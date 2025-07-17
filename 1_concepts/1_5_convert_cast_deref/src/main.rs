// Implement the following types:

//     EmailString - a type, which value can be only a valid email address string.
//     Random<T> - a smart pointer, which takes 3 values of the pointed-to type on creation and points to
//     one of them randomly every time is used.

// Provide conversion and Deref implementations for these types on your choice, to make their usage
// and interoperability with std types easy and ergonomic.

use std::ops::Deref;

use rand::{random, rngs::ThreadRng, seq::IndexedRandom, Rng};

#[derive(Debug)]
struct EmailString<'a>(Option<&'a str>);

impl<'a> EmailString<'a> {
    fn new(email: &'a str) -> Self {
        let prohibited_characters: Vec<char> = vec![' ', ',', ';', '<', '>', '[', ']', '"'];
        let split: Vec<&str> = email.split('@').collect();

        // I am sure there exists a trait for checking email for validity. But I am doing checks myself here. In order to practice
        if split.len() != 2
            || split[0].len() > 64
            || split[0].len() < 1
            || !split[1].contains('.')
            || split[0].chars().any(|c| prohibited_characters.contains(&c)) == true
            || split[1].chars().any(|c| prohibited_characters.contains(&c)) == true
        {
            EmailString(None)
        } else {
            EmailString(Some(email))
        }
    }
}

struct Random<T>(T)
where
    T: Copy;

impl<T: Copy> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy> Random<T> {
    fn new(val_1: T, val_2: T, val_3: T) -> Self {
        let items = vec![val_1, val_2, val_3];
        let mut rng = rand::rng();

        let random = items.choose(&mut rng).unwrap();
        Random(*random)
    }
}

fn main() {
    let test_email = EmailString::new("sssss@gmail.com");
    println!("email: {:?}", test_email);
}
