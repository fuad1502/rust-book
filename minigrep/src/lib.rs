use std::{error::Error, fs, env};

pub struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: &args[1],
            file_path: &args[2],
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    let matched_lines = if config.ignore_case {
        search_case_insensitive(config.query, &content)
    } else {
        search(config.query, &content)
    };

    for line in matched_lines {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matched_lines = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matched_lines = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick tree.";
        assert_eq!(vec!("safe, fast, productive."), search(query, content));
    }

    #[test]
    fn test_search_insensitive() {
        let query = "RUST";
        let content = "\
            Rust:
            safe, fast, productive.
            Pick tree.";
        assert_eq!(vec!("Rust:"), search_case_insensitive(query, content));
    }
}
