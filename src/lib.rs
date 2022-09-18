use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub querry: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let querry = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            querry,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.querry, &contents)
    } else {
        search(&config.querry, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(querry: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(querry) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    querry: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let querry = querry.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&querry) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let querry = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(querry, contents));
    }

    #[test]
    fn case_insensitive() {
        let querry = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(querry, contents)
        );
    }
}
