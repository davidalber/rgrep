struct Cacher<T>
    where T: Fn(u32) -> u32 {
    f: T,
    val: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(f: T) -> Cacher<T> {
        Cacher { f: f, val: None }
    }

    fn compute(&mut self, n: u32) -> u32 {
        if self.val.is_none() {
            self.val = Some((self.f)(n));
        }

        self.val.unwrap()
    }
}

fn main() {
    let mut square = Cacher::new(|n| {
        print!("squaring ");
        n*n
    });

    for i in 0..10 {
        for _ in 0..3 {
            println!("{}: {}", i, square.compute(i));
        }
    }
}
