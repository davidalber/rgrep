use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn from_args() -> Config {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 2 {
            panic!("Two arguments are required: query and file");
        }

        let query = args[0].clone();
        let filename = args[1].clone();
        Config { query, filename }
    }
}

fn main() {
    let config = Config::from_args();
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
}
