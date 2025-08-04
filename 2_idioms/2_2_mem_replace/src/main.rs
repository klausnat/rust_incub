use std::mem;

fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}

#[derive(Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Trinity<T> {
    fn rotate(&mut self) {
        std::mem::swap(&mut self.a, &mut self.c);
        std::mem::swap(&mut self.b, &mut self.a);
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: PartialEq> Solver<T> {
    fn resolve(&mut self) {
        let mut unsolved = mem::take(&mut self.unsolved);

        unsolved.retain_mut(|t| {
            for _ in 0..3 {
                if *t == self.expected {
                    return false;
                }
                t.rotate();
            }
            true
        });
        self.unsolved = unsolved;
    }
}
