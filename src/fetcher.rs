//! LeetCode API client with retry logic and user-friendly error handling

use std::fmt::{Display, Formatter};

use backon::{BlockingRetryable, ExponentialBuilder, Retryable};
use serde_json::Value;

use crate::error::{LeetCodeError, Result};

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";
const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;
const QUESTION_QUERY_OPERATION: &str = "questionData";

/// Fetch a single problem by its frontend ID
pub fn get_problem(frontend_question_id: u32) -> Result<Problem> {
    eprintln!(
        "📡 Fetching problem list to find problem #{}...",
        frontend_question_id
    );
    let problems = get_problems()?;

    let problem_stat = problems
        .stat_status_pairs
        .iter()
        .find(|p| p.stat.frontend_question_id == frontend_question_id)
        .ok_or(LeetCodeError::ProblemNotFound(frontend_question_id))?;

    if problem_stat.paid_only {
        return Err(LeetCodeError::ProblemNotFound(frontend_question_id));
    }

    let slug = problem_stat
        .stat
        .question_title_slug
        .as_ref()
        .ok_or(LeetCodeError::MissingTitleSlug(frontend_question_id))?;

    eprintln!("📡 Fetching problem details for '{}'...", slug);

    // Use backon for retry with exponential backoff
    (|| {
        fetch_problem_detail(
            &problem_stat.stat,
            slug,
            &problem_stat.difficulty.to_string(),
        )
    })
    .retry(ExponentialBuilder::default().with_max_times(3))
    .notify(|err: &LeetCodeError, dur: std::time::Duration| {
        eprintln!(
            "⚠️  Fetch problem detail failed: {}. Retrying in {:?}...",
            err, dur
        );
    })
    .call()
}

fn fetch_problem_detail(stat: &Stat, slug: &str, difficulty: &str) -> Result<Problem> {
    let headers = build_headers();
    let client = reqwest::blocking::Client::new();

    let response = client
        .post(GRAPHQL_URL)
        .headers(headers)
        .json(&Query::question_query(slug))
        .send()
        .map_err(|e| LeetCodeError::Http(e.to_string()))?;

    let raw_problem: RawProblem = response
        .json()
        .map_err(|e| LeetCodeError::Http(format!("Failed to parse response: {}", e)))?;

    let code_definition: Vec<CodeDefinition> =
        serde_json::from_str(&raw_problem.data.question.code_definition)
            .map_err(|e| LeetCodeError::CodeDefinitionParse(e.to_string()))?;

    let return_type = match serde_json::from_str::<Value>(&raw_problem.data.question.meta_data) {
        Ok(v) => v["return"]["type"].to_string().replace('"', ""),
        Err(e) => return Err(LeetCodeError::MetadataParse(e.to_string())),
    };

    Ok(Problem {
        title: stat.question_title.clone().unwrap_or_default(),
        title_slug: stat.question_title_slug.clone().unwrap_or_default(),
        code_definition,
        content: raw_problem.data.question.content,
        sample_test_case: raw_problem.data.question.sample_test_case,
        difficulty: difficulty.to_string(),
        question_id: stat.frontend_question_id,
        return_type,
    })
}

/// Fetch problem asynchronously with retry logic
pub async fn get_problem_async(problem_stat: StatWithStatus) -> Result<Problem> {
    if problem_stat.paid_only {
        return Err(LeetCodeError::ProblemNotFound(
            problem_stat.stat.frontend_question_id,
        ));
    }

    let slug =
        problem_stat
            .stat
            .question_title_slug
            .as_ref()
            .ok_or(LeetCodeError::MissingTitleSlug(
                problem_stat.stat.frontend_question_id,
            ))?;

    // Use backon for async retry
    (|| async { fetch_problem_detail_async(slug, &problem_stat).await })
        .retry(ExponentialBuilder::default().with_max_times(3))
        .sleep(tokio::time::sleep)
        .notify(|err: &LeetCodeError, dur: std::time::Duration| {
            eprintln!(
                "⚠️  Fetch problem #{} async failed: {}. Retrying in {:?}...",
                problem_stat.stat.frontend_question_id, err, dur
            );
        })
        .await
}

async fn fetch_problem_detail_async(slug: &str, problem_stat: &StatWithStatus) -> Result<Problem> {
    let resp = surf::post(GRAPHQL_URL)
        .body_json(&Query::question_query(slug))
        .map_err(|e| LeetCodeError::Http(e.to_string()))?;

    let raw_problem: RawProblem = resp
        .recv_json()
        .await
        .map_err(|e| LeetCodeError::Http(format!("Failed to receive JSON: {}", e)))?;

    let code_definition: Vec<CodeDefinition> =
        serde_json::from_str(&raw_problem.data.question.code_definition)
            .map_err(|e| LeetCodeError::CodeDefinitionParse(e.to_string()))?;

    let return_type = match serde_json::from_str::<Value>(&raw_problem.data.question.meta_data) {
        Ok(v) => v["return"]["type"].to_string().replace('"', ""),
        Err(e) => return Err(LeetCodeError::MetadataParse(e.to_string())),
    };

    Ok(Problem {
        title: problem_stat.stat.question_title.clone().unwrap_or_default(),
        title_slug: problem_stat
            .stat
            .question_title_slug
            .clone()
            .unwrap_or_default(),
        code_definition,
        content: raw_problem.data.question.content,
        sample_test_case: raw_problem.data.question.sample_test_case,
        difficulty: problem_stat.difficulty.to_string(),
        question_id: problem_stat.stat.frontend_question_id,
        return_type,
    })
}

/// Build HTTP headers for LeetCode API requests
fn build_headers() -> reqwest::header::HeaderMap {
    let mut h = reqwest::header::HeaderMap::new();
    h.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json, text/plain, */*"),
    );
    h.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36",
        ),
    );
    h.insert(
        "Accept-Language",
        reqwest::header::HeaderValue::from_static("en-US,en;q=0.9"),
    );
    h.insert(
        "Referer",
        reqwest::header::HeaderValue::from_static("https://leetcode.com/"),
    );

    // Add cookie if available (optional, only needed for authenticated endpoints)
    if let Ok(cookie) = std::env::var("LEETCODE_COOKIE")
        && !cookie.is_empty()
        && let Ok(cookie_value) = reqwest::header::HeaderValue::from_str(&cookie)
    {
        h.insert("Cookie", cookie_value);
    }

    h
}

/// Fetch all problems list
pub fn get_problems() -> Result<Problems> {
    eprintln!("📡 Fetching problem list from LeetCode...");

    // Use backon for retry
    (|| fetch_problems_impl())
        .retry(ExponentialBuilder::default().with_max_times(3))
        .notify(|err: &LeetCodeError, dur: std::time::Duration| {
            eprintln!(
                "⚠️  Fetch problems list failed: {}. Retrying in {:?}...",
                err, dur
            );
        })
        .call()
}

fn fetch_problems_impl() -> Result<Problems> {
    let headers = build_headers();
    let client = reqwest::blocking::Client::builder()
        .gzip(true)
        .build()
        .map_err(|e| LeetCodeError::HttpClientBuild(e.to_string()))?;

    let response = client
        .get(PROBLEMS_URL)
        .headers(headers)
        .send()
        .map_err(|e| LeetCodeError::Http(e.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        return Err(LeetCodeError::Http(format!("HTTP {}", status)));
    }

    response
        .json::<Problems>()
        .map_err(|e| LeetCodeError::Http(format!("Failed to decode JSON: {}", e)))
}

/// Problem data structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: u32,
    pub return_type: String,
}

/// Code definition for a specific language
#[derive(Serialize, Deserialize, Clone)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

/// GraphQL query structure
#[derive(Debug, Serialize, Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }
}

/// Raw problem response from GraphQL
#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

/// Problems list response
#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    #[allow(dead_code)]
    user_name: String,
    #[allow(dead_code)]
    num_solved: u32,
    #[allow(dead_code)]
    num_total: u32,
    #[allow(dead_code)]
    ac_easy: u32,
    #[allow(dead_code)]
    ac_medium: u32,
    #[allow(dead_code)]
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

/// Problem status with metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatWithStatus {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    #[allow(dead_code)]
    is_favor: bool,
    #[allow(dead_code)]
    frequency: u32,
    #[allow(dead_code)]
    progress: u32,
}

/// Problem statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stat {
    #[allow(dead_code)]
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    #[allow(dead_code)]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    pub question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    #[allow(dead_code)]
    question_hide: bool,
    #[allow(dead_code)]
    total_acs: u32,
    #[allow(dead_code)]
    total_submitted: u32,
    pub frontend_question_id: u32,
    #[allow(dead_code)]
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_display() {
        assert_eq!(format!("{}", Difficulty { level: 1 }), "Easy");
        assert_eq!(format!("{}", Difficulty { level: 2 }), "Medium");
        assert_eq!(format!("{}", Difficulty { level: 3 }), "Hard");
        assert_eq!(format!("{}", Difficulty { level: 4 }), "Unknown");
        assert_eq!(format!("{}", Difficulty { level: 0 }), "Unknown");
    }

    #[test]
    fn test_code_definition_deserialization() {
        let json = r#"[{"value":"rust","text":"Rust","defaultCode":"struct Solution;"}]"#;
        let defs: Vec<CodeDefinition> = serde_json::from_str(json).unwrap();
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].value, "rust");
        assert_eq!(defs[0].text, "Rust");
        assert_eq!(defs[0].default_code, "struct Solution;");
    }

    #[test]
    fn test_query_serialization() {
        let query = Query::question_query("two-sum");
        assert_eq!(query.operation_name, "questionData");
        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("two-sum"));
        assert!(json.contains("questionData"));
    }

    #[test]
    fn test_problems_deserialization() {
        let json = r#"{
            "user_name": "test",
            "num_solved": 10,
            "num_total": 100,
            "ac_easy": 5,
            "ac_medium": 3,
            "ac_hard": 2,
            "stat_status_pairs": [{
                "stat": {
                    "question_id": 1,
                    "question__article__slug": null,
                    "question__title": "Two Sum",
                    "question__title_slug": "two-sum",
                    "question__hide": false,
                    "total_acs": 1000,
                    "total_submitted": 2000,
                    "frontend_question_id": 1,
                    "is_new_question": false
                },
                "difficulty": {"level": 1},
                "paid_only": false,
                "is_favor": false,
                "frequency": 0,
                "progress": 0
            }]
        }"#;
        let problems: Problems = serde_json::from_str(json).unwrap();
        assert_eq!(problems.user_name, "test");
        assert_eq!(problems.num_solved, 10);
        assert_eq!(problems.stat_status_pairs.len(), 1);
        assert_eq!(problems.stat_status_pairs[0].stat.frontend_question_id, 1);
        assert!(!problems.stat_status_pairs[0].paid_only);
    }

    #[test]
    fn test_raw_problem_deserialization() {
        let json = r#"{
            "data": {
                "question": {
                    "content": "<p>Test content</p>",
                    "stats": "{\"totalAccepted\": \"100\"}",
                    "codeDefinition": "[{\"value\":\"rust\",\"text\":\"Rust\",\"defaultCode\":\"struct Solution;\"}]",
                    "sampleTestCase": "[1,2,3]\n4",
                    "metaData": "{\"return\":{\"type\":\"integer[]\"}}"
                }
            }
        }"#;
        let raw: RawProblem = serde_json::from_str(json).unwrap();
        assert_eq!(raw.data.question.content, "<p>Test content</p>");
        assert_eq!(raw.data.question.sample_test_case, "[1,2,3]\n4");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_build_headers() {
        // Save original cookie value
        let original = std::env::var("LEETCODE_COOKIE").ok();

        // Test 1: Without cookie (empty env var)
        unsafe {
            std::env::set_var("LEETCODE_COOKIE", "");
        }

        let headers = build_headers();

        // Verify required headers are present
        assert!(headers.contains_key("User-Agent"));
        assert!(headers.contains_key("Accept"));
        assert!(headers.contains_key("Referer"));

        // Cookie header should not be present when env var is empty
        assert!(!headers.contains_key("Cookie"));

        // Test 2: With valid cookie
        unsafe {
            std::env::set_var("LEETCODE_COOKIE", "test_cookie_value");
        }

        let headers = build_headers();

        // Cookie header should be present
        assert!(headers.contains_key("Cookie"));
        assert_eq!(
            headers.get("Cookie").unwrap().to_str().unwrap(),
            "test_cookie_value"
        );

        // Test 3: With invalid cookie characters (should not panic, just skip cookie)
        unsafe {
            std::env::set_var("LEETCODE_COOKIE", "invalid\n\rcookie");
        }

        let headers = build_headers();
        // Cookie should not be present when it contains invalid characters
        assert!(!headers.contains_key("Cookie"));

        // Restore original cookie
        unsafe {
            std::env::remove_var("LEETCODE_COOKIE");
            if let Some(cookie) = original {
                std::env::set_var("LEETCODE_COOKIE", cookie);
            }
        }
    }

    #[test]
    fn test_leetcode_error_display() {
        let err = LeetCodeError::ProblemNotFound(42);
        assert!(err.to_string().contains("Problem #42 not found"));

        let err = LeetCodeError::NoRustVersion(42);
        assert!(err.to_string().contains("Problem #42 has no Rust version"));
    }
}
