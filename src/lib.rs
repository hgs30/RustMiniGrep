use std::{env, error, fs};
use std::env::VarError;

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];


        let ignore_case = if args.len() == 4 {
            args[3] == "1"
        } else {
            match env::var("IGNORE_CASE") {
                Ok(val) => val == "1",
                Err(_) => false
            }
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_config_case_sensitive_env() {
        env::set_var("IGNORE_CASE", "0");

        let args = [
            String::from("rust"),
            String::from("to"),
            String::from("poem.text")];

        let config = Config::build(&args);

        assert!(!config.unwrap().ignore_case)
    }

    #[test]
    fn build_config_case_sensitive_arg() {
        let args = [
            String::from("rust"),
            String::from("to"),
            String::from("to"),
            String::from("0")];

        let config = Config::build(&args);

        assert!(!config.unwrap().ignore_case)
    }

    #[test]
    fn build_config_case_insensitive_env() {
        env::set_var("IGNORE_CASE", "1");

        let args = [
            String::from("rust"),
            String::from("to"),
            String::from("poem.text")];

        let config = Config::build(&args);

        assert!(config.unwrap().ignore_case)
    }

    #[test]
    fn build_config_case_insensitive_arg() {
        let args = [
            String::from("rust"),
            String::from("to"),
            String::from("to"),
            String::from("1")];

        let config = Config::build(&args);

        assert!(config.unwrap().ignore_case)
    }

    #[test]
    fn build_config_case_insensitive_arg_overrides_env() {
        env::set_var("IGNORE_CASE", "0");

        let args = [
            String::from("rust"),
            String::from("to"),
            String::from("to"),
            String::from("1")];

        let config = Config::build(&args);

        assert!(config.unwrap().ignore_case)
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}
