use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    f: T,
    vals: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(f: T) -> Cacher<T> {
        Cacher { f: f, vals: HashMap::new() }
    }

    fn compute(&mut self, n: u32) -> u32 {
        if !self.vals.contains_key(&n) {
            self.vals.insert(n, (self.f)(n));
        }

        self.vals.get(&n).unwrap().to_owned()
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
