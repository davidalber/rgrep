extern crate rgrep;

use std::env;
use std::process;

use rgrep::Config;

fn parse_args() -> Result<Config, &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();
    Config::new(args)
}

fn main() {
    let config = parse_args().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = rgrep::run(config){
        println!("Application error: {}", err);
        process::exit(1);
    };
}
