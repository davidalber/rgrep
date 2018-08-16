fn main() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}
