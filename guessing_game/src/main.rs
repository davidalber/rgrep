extern crate ansi_term;
extern crate rand;

use ansi_term::Colour::Cyan;
use ansi_term::Colour::Green;
use ansi_term::Colour::Red;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::env;

fn parse(args : Vec<String>) -> Result<(u32, u32), &'static str> {
    let low : u32 = match args.len() > 1 {
        false => 1,
        true  => args[1].trim().parse()
            .expect("The first argument must be a number."),
    };
    let high : u32 = match args.len() > 2 {
        false => 100,
        true  => args[2].trim().parse()
            .expect("The second argument must be a number."),
    };

    // Make sure that the range makes sense.
    return match low <= high {
        false => Err("High end of range cannot be smaller than low end of range"),
        true  => Ok((low, high)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (low, high) = parse(args)
        .expect("Reading input failed");

    println!("I'm thinking of a number between {} and {}. Guess the number!", low, high);

    let secret_number = rand::thread_rng().gen_range(low, high+1);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("{}", Cyan.paint("Too small!")),
            Ordering::Greater => println!("{}", Red.paint("Too large!")),
            Ordering::Equal   => {
                println!("{}", Green.paint("You win!"));
                break;
            },
        }
    }
}
