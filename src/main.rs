use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn from_args() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 2 {
            return Err("two arguments are required (query and file)");
        }

        let query = args[0].clone();
        let filename = args[1].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(())
}

fn main() {
    let config = Config::from_args().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config){
        println!("Application error: {}", err);
        process::exit(1);
    };
}
