fn foo(input: Option<i32>) -> Option<i32> {
    if input.is_none() {
        return None;
    }

    let input = input.unwrap();
    if input < 0 {
        return None;
    }
    Some(input)
}

fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
    match foo(input) {
        Some(n) => Ok(n),
        None => Err(ErrNegative),
    }
}

#[derive(Debug)]
struct ErrNegative;

fn main() {
    println!("{:?}", foo(Some(0)));
    println!("{:?}", foo(None));
    println!("{:?}", bar(Some(42)));
    println!("{:?}", bar(None));
}
