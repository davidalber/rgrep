use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Two arguments are required: query and file");
    }

    let query = &args[0];
    let file = &args[1];

    println!("Searching for {}", query);
    println!("In file {}", file);
}
