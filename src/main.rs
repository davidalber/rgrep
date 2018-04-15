extern crate rgrep;

use std::process;

use rgrep::Config;

fn main() {
    let config = Config::from_args().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = rgrep::run(config){
        println!("Application error: {}", err);
        process::exit(1);
    };
}
