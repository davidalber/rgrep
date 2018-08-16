fn foo(input: Option<i32>) -> Option<i32> {
    input.filter(|i| i >= &0)
}

fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
    input.ok_or(ErrNegative)
}

#[derive(Debug)]
struct ErrNegative;

fn main() {
    println!("{:?}", foo(Some(0)));
    println!("{:?}", foo(None));
    println!("{:?}", bar(Some(42)));
    println!("{:?}", bar(None));
}
