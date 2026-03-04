extern crate reqwest;
extern crate serde_json;

use std::fmt::{Display, Error, Formatter};

use serde_json::Value;

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

pub fn get_problem(frontend_question_id: u32) -> Option<Problem> {
    let problems = match get_problems() {
        Some(p) => p,
        None => {
            println!("Failed to fetch problems list");
            return None;
        }
    };
    for problem in problems.stat_status_pairs.iter() {
        if problem.stat.frontend_question_id == frontend_question_id {
            if problem.paid_only {
                return None;
            }
            println!("getting problem ...");
            let client = reqwest::blocking::Client::new();
            let slug = match problem.stat.question_title_slug.as_ref() {
                Some(s) => s,
                None => {
                    println!("Problem {} has no title slug", frontend_question_id);
                    return None;
                }
            };
            let send_res = client
                .post(GRAPHQL_URL)
                .json(&Query::question_query(slug))
                .send();
            let resp: RawProblem = match send_res {
                Ok(r) => match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        println!("Failed to parse problem response JSON: {}", e);
                        return None;
                    }
                },
                Err(e) => {
                    println!("HTTP request failed: {}", e);
                    return None;
                }
            };

            let code_definition: Vec<CodeDefinition> =
                match serde_json::from_str(&resp.data.question.code_definition) {
                    Ok(cd) => cd,
                    Err(e) => {
                        println!("Failed to parse code definition: {}", e);
                        return None;
                    }
                };

            let return_type = match serde_json::from_str::<Value>(&resp.data.question.meta_data) {
                Ok(v) => v["return"]["type"].to_string().replace('"', ""),
                Err(e) => {
                    println!("Failed to parse meta_data: {}", e);
                    return None;
                }
            };

            return Some(Problem {
                title: problem.stat.question_title.clone().unwrap_or_default(),
                title_slug: problem.stat.question_title_slug.clone().unwrap_or_default(),
                code_definition,
                content: resp.data.question.content,
                sample_test_case: resp.data.question.sample_test_case,
                difficulty: problem.difficulty.to_string(),
                question_id: problem.stat.frontend_question_id,
                return_type,
            });
        }
    }
    None
}

pub async fn get_problem_async(problem_stat: StatWithStatus) -> Option<Problem> {
    if problem_stat.paid_only {
        println!(
            "Problem {} is paid-only",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let resp = surf::post(GRAPHQL_URL).body_json(&Query::question_query(
        problem_stat.stat.question_title_slug.as_ref().unwrap(),
    ));
    if resp.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let recv = resp.unwrap().recv_json().await;
    if recv.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }
    let resp: RawProblem = recv.unwrap();

    let code_definition: Vec<CodeDefinition> =
        match serde_json::from_str(&resp.data.question.code_definition) {
            Ok(cd) => cd,
            Err(e) => {
                println!("Failed to parse code definition: {}", e);
                return None;
            }
        };

    let return_type = match serde_json::from_str::<Value>(&resp.data.question.meta_data) {
        Ok(v) => v["return"]["type"].to_string().replace('"', ""),
        Err(e) => {
            println!("Failed to parse meta_data: {}", e);
            return None;
        }
    };

    Some(Problem {
        title: problem_stat.stat.question_title.clone().unwrap_or_default(),
        title_slug: problem_stat
            .stat
            .question_title_slug
            .clone()
            .unwrap_or_default(),
        code_definition,
        content: resp.data.question.content,
        sample_test_case: resp.data.question.sample_test_case,
        difficulty: problem_stat.difficulty.to_string(),
        question_id: problem_stat.stat.frontend_question_id,
        return_type,
    })
}

pub fn get_problems() -> Option<Problems> {
    println!("getting all problems ...");
    let cookie = match std::env::var("LEETCODE_COOKIE") {
        Ok(c) if !c.is_empty() => c,
        _ => {
            println!("Error: LEETCODE_COOKIE is not set or empty in .env file or environment");
            println!("Please set a valid LeetCode session cookie to fetch problems");
            return None;
        }
    };

    let headers = {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
            ),
        );

        h.insert(
            "Connection",
            reqwest::header::HeaderValue::from_static("keep-alive"),
        );
        h.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36",
            ),
        );
        h.insert(
            "Accept-Encoding",
            reqwest::header::HeaderValue::from_static("gzip, deflate, br, zstd"),
        );
        h.insert(
            "Accept-Language",
            reqwest::header::HeaderValue::from_static("en-US,en;q=0.9"),
        );
        h.insert(
            "Cache-Control",
            reqwest::header::HeaderValue::from_static("no-cache"),
        );
        h.insert(
            "Pragma",
            reqwest::header::HeaderValue::from_static("no-cache"),
        );
        h.insert(
            "Priority",
            reqwest::header::HeaderValue::from_static("u=0, i"),
        );
        h.insert(
            "Sec-CH-UA",
            reqwest::header::HeaderValue::from_static(
                "\"Not(A:Brand\";v=\"99\", \"Google Chrome\";v=\"133\", \"Chromium\";v=\"133\"",
            ),
        );
        h.insert(
            "Sec-CH-UA-Mobile",
            reqwest::header::HeaderValue::from_static("?0"),
        );
        h.insert(
            "Sec-CH-UA-Platform",
            reqwest::header::HeaderValue::from_static("\"macOS\""),
        );
        h.insert(
            "Sec-Fetch-Dest",
            reqwest::header::HeaderValue::from_static("document"),
        );
        h.insert(
            "Sec-Fetch-Mode",
            reqwest::header::HeaderValue::from_static("navigate"),
        );
        h.insert(
            "Sec-Fetch-Site",
            reqwest::header::HeaderValue::from_static("none"),
        );
        h.insert(
            "Sec-Fetch-User",
            reqwest::header::HeaderValue::from_static("?1"),
        );
        h.insert(
            "Upgrade-Insecure-Requests",
            reqwest::header::HeaderValue::from_static("1"),
        );

        h.insert(
            "Host",
            reqwest::header::HeaderValue::from_static("leetcode.com"),
        );
        h.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_str(&cookie).unwrap(),
        );
        h
    };
    let client = match reqwest::blocking::Client::builder().gzip(true).build() {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to build HTTP client: {}", e);
            return None;
        }
    };
    let get = client.get(PROBLEMS_URL).headers(headers);
    // println!("Get: {:?}", get);
    let response = match get.send() {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to fetch problems URL: {}", e);
            return None;
        }
    };
    let status = response.status();
    println!("Response status: {}", status);
    // println!("Response: {:?}", response);
    match response.json::<Problems>() {
        Ok(p) => Some(p),
        Err(e) => {
            println!("Failed to decode problems JSON: {}", e);
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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
    fn test_get_problems_missing_cookie() {
        // Save original cookie value
        let original = std::env::var("LEETCODE_COOKIE").ok();

        // Remove cookie
        unsafe {
            std::env::remove_var("LEETCODE_COOKIE");
        }

        // Should return None when cookie is missing
        let result = get_problems();
        assert!(result.is_none());

        // Restore original cookie
        unsafe {
            if let Some(cookie) = original {
                std::env::set_var("LEETCODE_COOKIE", cookie);
            }
        }
    }

    #[test]
    fn test_get_problems_empty_cookie() {
        // Save original cookie value
        let original = std::env::var("LEETCODE_COOKIE").ok();

        // Set empty cookie
        unsafe {
            std::env::set_var("LEETCODE_COOKIE", "");
        }

        // Should return None when cookie is empty
        let result = get_problems();
        assert!(result.is_none());

        // Restore original cookie
        unsafe {
            std::env::remove_var("LEETCODE_COOKIE");
            if let Some(cookie) = original {
                std::env::set_var("LEETCODE_COOKIE", cookie);
            }
        }
    }
}
