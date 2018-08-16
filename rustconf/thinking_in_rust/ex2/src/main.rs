fn main() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();
    loop {
        match iter.next() {
            Some(v) => println!("{}", v),
            None => break,
        }
    }
}
