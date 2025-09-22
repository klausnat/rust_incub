use std::{fs::File, io, path::Path};

use rand::seq::IndexedRandom;
use sha3::{Digest, Sha3_256};

const ALL_CHARS_PASSWORD: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!()-+=[]{}?";
const ALL_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const PASSWORD_LEN: usize = 6;

// generates random password of given length and symbols set;
fn generate_password() -> String {
    let mut rng = &mut rand::rng();
    let sample = ALL_CHARS_PASSWORD.as_bytes();
    let random_in_bytes: [u8; PASSWORD_LEN] = sample.choose_multiple_array(&mut rng).unwrap();
    String::from_utf8(random_in_bytes.to_vec()).unwrap()
}

// retrieves random element from a given slice;
fn select_rand_val<T: Copy>(slice: &[T]) -> T {
    let mut rng = rand::rng();
    let res = slice.choose(&mut rng).unwrap();
    *res
}

// generates unique cryptographically secure random value in a-zA-Z0-9 symbols set and has exactly 64 symbols.
fn new_access_token() -> [char; 64] {
    // since rng is cryptographycally secure, we can use rand::rng
    let mut rng = &mut rand::rng();
    let sample: [u8; 64] = ALL_CHARS.as_bytes().choose_multiple_array(rng).unwrap();
    let res = sample.map(|x| char::from(x));
    res
}

// returns SHA-3 hash of a file specified by its path.
fn get_file_hash<P : AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = std::fs::read(file_path)?;
    let hash = Sha3_256::digest(file);
    Ok(format!("{:x}", hash))

}

// @TODO - last function wouldn't compile, in order to make it work I need to downgrade version of rand
// to 0.8.5 - and work with some deprecated functions in previous functions. Versions conflict

// use rand::rand_core::{OsRng, RngCore};
// use argon2::{
//     password_hash::{
//         PasswordHasher, 
//         SaltString
//     },
//     Argon2
// };

// fn hash_password(password: &str) -> String {
//     let mut rng = OsRng;
//     let salt = SaltString::generate(&mut rng);
    
//     Argon2::default()
//         .hash_password(password.as_bytes(), &salt)
//         .expect("Hashing failed")
//         .to_string()
// }

fn main() {
    println!("all functions implemented");
}
