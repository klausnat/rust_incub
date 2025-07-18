// Implement the following types:

//     EmailString - a type, which value can be only a valid email address string.
//     Random<T> - a smart pointer, which takes 3 values of the pointed-to type on creation and points to
//     one of them randomly every time is used.

// Provide conversion and Deref implementations for these types on your choice, to make their usage
// and interoperability with std types easy and ergonomic.

use std::{ops::Deref, str::FromStr};

use rand::seq::IndexedRandom;

#[derive(Debug)]

struct EmailString(Option<String>);

impl EmailString {
    fn new(email: &str) -> Self {
        let prohibited_characters: Vec<char> = vec![' ', ',', ';', '<', '>', '[', ']', '"'];
        let split: Vec<&str> = email.split('@').collect();

        if split.len() != 2
            || split[0].len() > 64
            || split[0].len() < 1
            || !split[1].contains('.')
            || split[0].chars().any(|c| prohibited_characters.contains(&c)) == true
            || split[1].chars().any(|c| prohibited_characters.contains(&c)) == true
        {
            EmailString(None)
        } else {
            EmailString(Some(String::from(email)))
        }
    }
}

impl AsRef<Option<String>> for EmailString {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}

// Deref should only be implemented for smart pointers. Can we consider our EmailString a smart pointer?
impl Deref for EmailString {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for EmailString {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EmailString::new(s))
    }
}

// conversion can not fail, since we have Option type inside, and fn new taking care of email validation
impl From<&str> for EmailString {
    fn from(email: &str) -> Self {
        EmailString::new(email)
    }
}

struct Random<T> {
    values: [T; 3],
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut rng = rand::rng();
        let random = &self.values.choose(&mut rng).unwrap();
        *random
    }
}

impl<T: Copy> Random<T> {
    fn new(val_1: T, val_2: T, val_3: T) -> Self {
        Random {
            values: [val_1, val_2, val_3],
        }
    }
}

impl<T: Clone> From<[T; 3]> for Random<T> {
    fn from(values: [T; 3]) -> Self {
        Random { values }
    }
}

fn main() {
    let test_email = EmailString::new("sssss@gmail.com");
    println!("email: {:?}", test_email);

    let test_rand = Random::new(2, 3, 6);
    println!("random 1: {}", *test_rand);
    println!("random 2: {}", *test_rand);
    println!("random 3: {}", *test_rand);
}
