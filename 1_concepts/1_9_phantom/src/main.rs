// Implement a Fact<T> type which returns some random fact about T type that Fact<T> is implemented for.

// let f: Fact<Vec<T>> = Fact::new();
// println!("Fact about Vec: {}", f.fact());
// println!("Fact about Vec: {}", f.fact());

// Fact about Vec: Vec is heap-allocated.
// Fact about Vec: Vec may re-allocate on growing.
use std::marker::PhantomData;
use rand::seq::IndexedMutRandom;

struct Fact<T> {
    _phtype: PhantomData<T>,
}

impl<T> Fact<T> {
    fn new() -> Self {
        Fact {
            _phtype: PhantomData,
        }
    }
}

impl<T> Fact<Vec<T>> {
    fn fact(&self) -> &'static str {
        let mut facts = [
            "Vec is heap-allocated.",
            "There is a vec! macro.",
            "The Vec type allows access to values by index, because it implements the Index trait.",
        ];
        facts.choose_mut(&mut rand::rng()).unwrap()
    }
}

impl Fact<String> {
    fn fact(&self) -> &'static str {
        let mut facts = [
            "String is UTF-8–encoded and growable",
            "String implements Deref<Target = str>",
            "A String is made up of three components: a pointer to some bytes, a length, and a capacity.",
        ];
        facts.choose_mut(&mut rand::rng()).unwrap()
    }
}

fn main() {
    let f: Fact<Vec<i32>> = Fact::new();
    let s: Fact<String> = Fact::new();
    println!("Random fact about Vec: {}", f.fact());
    println!("Random fact about String: {}", s.fact());
}
