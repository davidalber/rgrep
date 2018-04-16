use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("two arguments are required (query and file)");
        }

        let query = args[0].clone();
        let filename = args[1].clone();
        Ok(Config { query, filename })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.split('\n').filter(|l| l.contains(query)).collect()
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
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
            }).is_err()
        );
    }

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
