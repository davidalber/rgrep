use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Two arguments are required: query and file");
    }

    let query = &args[0];
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("{}", contents);
}
