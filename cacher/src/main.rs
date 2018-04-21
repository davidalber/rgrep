use std::collections::HashMap;

struct Cacher<T, A>
    where T: Fn(&A) -> A {
    f: T,
    vals: HashMap<A, A>,
}

impl<T, A> Cacher<T, A>
    where T: Fn(&A) -> A,
          A: std::cmp::Eq + std::hash::Hash + std::clone::Clone {
    fn new(f: T) -> Cacher<T, A> {
        Cacher { f: f, vals: HashMap::new() }
    }

    fn compute(&mut self, n: A) -> &A {
        if !self.vals.contains_key(&n) {
            let val = (self.f)(&n);
            self.vals.insert(n.clone(), val);
        }

        self.vals.get(&n).unwrap()
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
