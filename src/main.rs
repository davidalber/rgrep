use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse_config() -> (String, String) {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Two arguments are required: query and file");
    }

    let query = args[0].to_owned();
    let filename = args[1].to_owned();
    (query, filename)
}

fn main() {
    let (query, filename) = parse_config();
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
}
