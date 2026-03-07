//! LeetCode Rust CLI - Generate solution templates from LeetCode
//!
//! Usage: leetcode-rust [COMMAND]
//!
//! Commands:
//!   get      Get a specific problem by ID
//!   random   Get a random problem
//!   all      Initialize all problems (warning: fetches all problems!)
//!   solve    Move a problem from problem/ to solution/
//!   list     List all initialized problems
//!   help     Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate dotenv;

mod error;
mod fetcher;

use std::{
    fs,
    fs::File,
    io::{BufRead, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use futures::{
    executor::{ThreadPool, block_on},
    future::join_all,
    task::SpawnExt,
};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;

use crate::{
    error::{LeetCodeError, Result},
    fetcher::{CodeDefinition, Problem},
};

/// LeetCode Rust CLI - Generate solution templates from LeetCode
#[derive(Parser)]
#[command(name = "leetcode-rust")]
#[command(about = "Generate LeetCode solution templates in Rust")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a specific problem by ID
    Get {
        /// Problem ID to fetch
        id: u32,
    },
    /// Get a random problem
    Random,
    /// Initialize all problems (warning: fetches all problems!)
    All,
    /// Move a problem from problem/ to solution/
    Solve {
        /// Problem ID to solve
        id: u32,
    },
    /// List all initialized problems
    List,
}

fn main() {
    println!("🦀 Welcome to leetcode-rust CLI\n");

    // Load environment variables
    if let Err(e) = dotenv() {
        eprintln!("⚠️  Warning: Failed to load .env file: {}", e);
    }

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Get { id } => handle_get(id),
        Commands::Random => handle_random(),
        Commands::All => handle_all(),
        Commands::Solve { id } => handle_solve(id),
        Commands::List => handle_list(),
    };

    match result {
        Ok(()) => {
            println!("\n✅ Done! Thanks for using leetcode-rust CLI.\n");
        }
        Err(e) => {
            eprintln!("\n❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle the `get` command - fetch a specific problem
fn handle_get(id: u32) -> Result<()> {
    let initialized_ids = get_initialized_ids("./src/problem/mod.rs")?;

    if initialized_ids.contains(&id) {
        return Err(LeetCodeError::AlreadyInitialized(id));
    }

    let problem = fetcher::get_problem(id)?;
    let code = find_rust_code(&problem)?;

    deal_problem(&problem, &code, true)?;

    println!(
        "✅ Successfully created problem #{}: {} [{}]",
        problem.question_id, problem.title, problem.difficulty
    );

    Ok(())
}

/// Handle the `random` command - fetch a random problem
fn handle_random() -> Result<()> {
    let initialized_ids = get_initialized_ids("./src/problem/mod.rs")?;

    println!("🎲 Finding a random problem...");

    let problems = fetcher::get_problems()?;
    let available_problems: Vec<_> = problems
        .stat_status_pairs
        .iter()
        .filter(|p| !p.paid_only && !initialized_ids.contains(&p.stat.frontend_question_id))
        .collect();

    if available_problems.is_empty() {
        return Err(LeetCodeError::InvalidInput(
            "No available problems found".to_string(),
        ));
    }

    use rand::Rng;
    let mut rng = rand::rng();
    let selected = available_problems[rng.random_range(0..available_problems.len())];
    let id = selected.stat.frontend_question_id;

    println!("🎲 Selected random problem: #{}\n", id);

    let problem = fetcher::get_problem(id)?;
    let code = find_rust_code(&problem)?;

    deal_problem(&problem, &code, true)?;

    println!(
        "✅ Successfully created problem #{}: {} [{}]",
        problem.question_id, problem.title, problem.difficulty
    );

    Ok(())
}

/// Handle the `all` command - fetch all problems
fn handle_all() -> Result<()> {
    println!("⚠️  Warning: This will fetch ALL LeetCode problems.");
    println!("   This may take a while and make many API requests.\n");

    let initialized_ids = get_initialized_ids("./src/problem/mod.rs")?;
    let problems = fetcher::get_problems()?;

    let to_fetch: Vec<_> = problems
        .stat_status_pairs
        .into_iter()
        .filter(|p| !p.paid_only && !initialized_ids.contains(&p.stat.frontend_question_id))
        .collect();

    let total = to_fetch.len();
    println!("📊 Found {} problems to fetch\n", total);

    if total == 0 {
        println!("✅ All problems are already initialized!");
        return Ok(());
    }

    // Create progress bar
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let pool = ThreadPool::new().map_err(|e| LeetCodeError::AsyncRuntime(e.to_string()))?;

    let mod_file_addon = Arc::new(Mutex::new(Vec::new()));
    let pb = Arc::new(Mutex::new(pb));

    let mut tasks = vec![];

    for problem_stat in to_fetch {
        let mod_file_addon = mod_file_addon.clone();
        let pb = pb.clone();

        let task = pool
            .spawn_with_handle(async move {
                let id = problem_stat.stat.frontend_question_id;

                match fetcher::get_problem_async(problem_stat.clone()).await {
                    Ok(problem) => {
                        match find_rust_code(&problem) {
                            Ok(code) => {
                                // Generate module line
                                let file_name = format!(
                                    "p{:04}_{}",
                                    problem.question_id,
                                    problem.title_slug.replace('-', "_")
                                );
                                mod_file_addon
                                    .lock()
                                    .unwrap()
                                    .push(format!("pub mod {file_name};"));

                                // Write problem file (async block for consistency)
                                let result = async { deal_problem(&problem, &code, false) }.await;

                                if let Err(e) = result {
                                    eprintln!("\n⚠️  Failed to write problem #{}: {}", id, e);
                                }
                            }
                            Err(e) => {
                                eprintln!("\n⚠️  Problem #{} has no Rust version: {}", id, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("\n⚠️  Failed to fetch problem #{}: {}", id, e);
                    }
                }

                pb.lock().unwrap().inc(1);
            })
            .map_err(|e| LeetCodeError::AsyncRuntime(e.to_string()))?;

        tasks.push(task);
    }

    block_on(join_all(tasks));

    pb.lock()
        .unwrap()
        .finish_with_message("Done fetching problems");

    // Write all module declarations at once
    let mut lib_file = fs::OpenOptions::new()
        .append(true)
        .open("./src/problem/mod.rs")
        .map_err(|e| LeetCodeError::file_operation("open", "./src/problem/mod.rs", e))?;

    let declarations = mod_file_addon.lock().unwrap().join("\n");
    if !declarations.is_empty() {
        writeln!(lib_file, "\n{}", declarations)
            .map_err(|e| LeetCodeError::file_operation("write", "./src/problem/mod.rs", e))?;
    }

    println!("\n✅ Finished fetching all problems!");

    Ok(())
}

/// Handle the `solve` command - move problem to solution/
fn handle_solve(id: u32) -> Result<()> {
    println!("📝 Moving problem #{} to solution/...", id);

    let problem = fetcher::get_problem(id)?;

    let file_name = format!(
        "p{:04}_{}",
        problem.question_id,
        problem.title_slug.replace('-', "_")
    );
    let file_path = Path::new("./src/problem").join(format!("{file_name}.rs"));

    // Check problem/ existence
    if !file_path.exists() {
        return Err(LeetCodeError::ProblemNotExist(id));
    }

    // Check solution/ no existence
    let solution_name = format!(
        "s{:04}_{}",
        problem.question_id,
        problem.title_slug.replace('-', "_")
    );
    let solution_path = Path::new("./src/solution").join(format!("{solution_name}.rs"));

    if solution_path.exists() {
        return Err(LeetCodeError::SolutionExists(id));
    }

    // Rename/move file
    fs::rename(&file_path, &solution_path)
        .map_err(|e| LeetCodeError::file_operation("rename", &file_path, e))?;

    // Remove from problem/mod.rs
    let mod_file = "./src/problem/mod.rs";
    let target_line = format!("pub mod {file_name};");
    let lines: Vec<String> = std::io::BufReader::new(
        File::open(mod_file).map_err(|e| LeetCodeError::file_operation("open", mod_file, e))?,
    )
    .lines()
    .map(|x| x.unwrap())
    .filter(|x| *x != target_line)
    .collect();

    fs::write(mod_file, lines.join("\n"))
        .map_err(|e| LeetCodeError::file_operation("write", mod_file, e))?;

    // Insert into solution/mod.rs
    let mut lib_file = fs::OpenOptions::new()
        .append(true)
        .open("./src/solution/mod.rs")
        .map_err(|e| LeetCodeError::file_operation("open", "./src/solution/mod.rs", e))?;

    writeln!(lib_file, "pub mod {solution_name};")
        .map_err(|e| LeetCodeError::file_operation("write", "./src/solution/mod.rs", e))?;

    println!("✅ Successfully moved problem #{} to solution/", id);

    Ok(())
}

/// Handle the `list` command - list initialized problems
fn handle_list() -> Result<()> {
    let initialized_ids = get_initialized_ids("./src/problem/mod.rs")?;

    if initialized_ids.is_empty() {
        println!("No problems initialized yet.");
        println!("\nUse one of the following commands to get started:");
        println!("  leetcode-rust get <id>    - Get a specific problem");
        println!("  leetcode-rust random      - Get a random problem");
        return Ok(());
    }

    println!(
        "📚 Initialized problems ({} total):\n",
        initialized_ids.len()
    );

    // Group by hundreds for better readability
    let mut ids = initialized_ids;
    ids.sort_unstable();

    for chunk in ids.chunks(10) {
        let ids_str: Vec<String> = chunk.iter().map(|id| format!("#{}", id)).collect();
        println!("  {}", ids_str.join(", "));
    }

    Ok(())
}

/// Find Rust code definition in a problem
fn find_rust_code(problem: &Problem) -> Result<CodeDefinition> {
    problem
        .code_definition
        .iter()
        .find(|d| d.value == "rust")
        .cloned()
        .ok_or(LeetCodeError::NoRustVersion(problem.question_id))
}

/// Get list of already initialized problem IDs
fn get_initialized_ids(path: &str) -> Result<Vec<u32>> {
    let content =
        fs::read_to_string(path).map_err(|e| LeetCodeError::file_operation("read", path, e))?;
    let id_pattern = Regex::new(r"p(\d{4})_")?;

    let mut ret = vec![];
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("//")
            && let Some(id) = id_pattern.captures(trimmed)
            && let Ok(id_num) = id.get(1).unwrap().as_str().parse::<u32>()
        {
            ret.push(id_num);
        }
    }

    Ok(ret)
}

/// Parse extra use statements based on code content
fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse crate::util::linked_list::{ListNode, to_list};");
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse crate::util::tree::{TreeNode, to_tree};");
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse crate::util::point::Point;");
    }
    extra_use_line
}

/// Build problem link
fn parse_problem_link(problem: &Problem) -> String {
    format!("https://leetcode.com/problems/{}/", problem.title_slug)
}

/// Build discuss link
fn parse_discuss_link(problem: &Problem) -> String {
    format!(
        "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
        problem.title_slug
    )
}

/// Insert default return value in code template
fn insert_return_in_code(return_type: &str, code: &str) -> String {
    let re = Regex::new(r"\{\s*}").unwrap();
    match return_type {
        "ListNode" => re
            .replace(code, "{\n        Some(Box::new(ListNode::new(0)))\n    }")
            .to_string(),
        "ListNode[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "TreeNode" => re
            .replace(
                code,
                "{\n        Some(Rc::new(RefCell::new(TreeNode::new(0))))\n    }",
            )
            .to_string(),
        "boolean" => re.replace(code, "{\n        false\n    }").to_string(),
        "character" => re.replace(code, "{\n        '0'\n    }").to_string(),
        "character[][]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "double" => re.replace(code, "{\n        0f64\n    }").to_string(),
        "double[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "int[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "integer" => re.replace(code, "{\n        0\n    }").to_string(),
        "integer[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "integer[][]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<String>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<TreeNode>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<boolean>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<double>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<integer>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<list<integer>>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<list<string>>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "list<string>" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "null" => code.to_string(),
        "string" => re
            .replace(code, "{\n        String::new()\n    }")
            .to_string(),
        "string[]" => re.replace(code, "{\n        vec![]\n    }").to_string(),
        "void" => code.to_string(),
        "NestedInteger" => code.to_string(),
        "Node" => code.to_string(),
        _ => code.to_string(),
    }
}

/// Clean HTML content for Rust comments
fn build_desc(content: &str) -> String {
    content
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("</p>", "")
        .replace("<p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", "")
        .replace("<code>", "")
        .replace("</code>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<sub>", "")
        .replace("</sub>", "")
        .replace("</sup>", "")
        .replace("<sup>", "^")
        .replace("&nbsp;", " ")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&minus;", "-")
        .replace("&#39;", "'")
        .replace("\n\n", "\n")
        .replace('\n', "\n * ")
}

/// Generate problem file from template
fn deal_problem(problem: &Problem, code: &CodeDefinition, write_mod_file: bool) -> Result<()> {
    let file_name = format!(
        "p{:04}_{}",
        problem.question_id,
        problem.title_slug.replace('-', "_")
    );
    let file_path = Path::new("./src/problem").join(format!("{file_name}.rs"));

    let template = fs::read_to_string("./template.rs")
        .map_err(|e| LeetCodeError::Template(format!("Failed to read template.rs: {}", e)))?;

    let source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace("__PROBLEM_DESC__", &build_desc(&problem.content))
        .replace(
            "__PROBLEM_DEFAULT_CODE__",
            &insert_return_in_code(&problem.return_type, &code.default_code),
        )
        .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
        .replace("__PROBLEM_LINK__", &parse_problem_link(problem))
        .replace("__DISCUSS_LINK__", &parse_discuss_link(problem));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .map_err(|e| LeetCodeError::file_operation("create", &file_path, e))?;

    file.write_all(source.as_bytes())
        .map_err(|e| LeetCodeError::file_operation("write", &file_path, e))?;

    drop(file);

    if write_mod_file {
        let mut lib_file = fs::OpenOptions::new()
            .append(true)
            .open("./src/problem/mod.rs")
            .map_err(|e| LeetCodeError::file_operation("open", "./src/problem/mod.rs", e))?;

        writeln!(lib_file, "pub mod {file_name};")
            .map_err(|e| LeetCodeError::file_operation("write", "./src/problem/mod.rs", e))?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_initialized_ids() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("test_get_initialized_ids");
        let mut file = fs::File::create(path.clone()).unwrap();
        let content = r#"
        // pub mod p0001_two_sum;
        pub mod p0002_add_two_numbers;
        pub mod p0003_longest_substring_without_repeating_characters;"#;
        writeln!(file, "{content}").unwrap();
        let ids = get_initialized_ids(path.to_str().unwrap()).unwrap();
        println!("{ids:?}");
        assert!(ids.len() == 2);
        assert!(ids[0] == 2);
        assert!(ids[1] == 3);
    }

    #[test]
    fn test_parse_extra_use() {
        let code_with_listnode = "pub struct ListNode";
        assert!(parse_extra_use(code_with_listnode).contains("linked_list"));

        let code_with_treenode = "pub struct TreeNode";
        assert!(parse_extra_use(code_with_treenode).contains("tree"));

        let code_with_point = "pub struct Point";
        assert!(parse_extra_use(code_with_point).contains("point"));

        let code_empty = "struct Solution;";
        assert!(parse_extra_use(code_empty).is_empty());
    }

    #[test]
    fn test_insert_return_in_code() {
        let code = "impl Solution { pub fn test() {} }";
        let result = insert_return_in_code("integer", code);
        assert!(result.contains("0"));

        let result = insert_return_in_code("string", code);
        assert!(result.contains("String::new()"));

        let result = insert_return_in_code("int[]", code);
        assert!(result.contains("vec![]"));
    }

    #[test]
    fn test_build_desc() {
        let html = "<p>Hello <strong>world</strong></p>";
        let result = build_desc(html);
        assert!(!result.contains("<p>"));
        assert!(!result.contains("<strong>"));
        assert!(result.contains("Hello"));
        assert!(result.contains("world"));
    }
}
