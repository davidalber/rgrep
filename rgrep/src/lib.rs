use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("two arguments are required (query and file)");
        }

        let query = args[0].clone();
        let filename = args[1].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.split('\n').filter(|l| l.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.split("\n").filter(|l| l.to_lowercase().contains(&query)).collect()
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_ok() {
        let config = Config::new(vec![String::from("a"), String::from("b")]).unwrap();
        assert_eq!(
            config,
            Config {
                query: String::from("a"),
                filename: String::from("b"),
                case_sensitive: true,
            }
        );
    }

    #[test]
    fn config_too_few_arguments() {
        assert!(Config::new(vec![String::from("a")]).is_err());
    }

    #[test]
    fn config_too_many_arguments() {
        assert!(
            Config::new(vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
            ]).is_err()
        );
    }

    #[test]
    fn run_bad_filename() {
        assert!(
            run(Config {
                query: String::from("q"),
                filename: String::from("fakefile.txt"),
                case_sensitive: true,
            }).is_err()
        );
    }

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_multiple_results() {
        let query = "s";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn search_no_results() {
        let query = "bar";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(Vec::<String>::new(), search(query, contents));
    }

    #[test]
    fn search_case_insensitive_multiple_results() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
