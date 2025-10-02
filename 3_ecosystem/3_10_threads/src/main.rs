use crossbeam_channel::unbounded;
use rand::Rng;
use rayon::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

type Matrix = Vec<Vec<u8>>;

fn generate_square_matrix() -> Matrix {
    // this random generator connected to current thread
    // each call would change rng, so that is why it is mut
    let mut rng = rand::rng();
    let mut matrix = Vec::with_capacity(64);

    for _ in 0..64 {
        let row: Vec<u8> = (0..64).map(|_| rng.random_range(0..=255)).collect();
        matrix.push(row);
    }

    matrix
}

fn sum_of_matrix_elements(matrix: Matrix) -> u32 {
    matrix
        .par_iter()
        .map(|row| row.par_iter().map(|&x| x as u32).sum::<u32>())
        .sum()
}

fn main() {
    let start = Instant::now();
    let (tx, rx) = unbounded();

    // Clone receiver for second consumer (crossbeam allows to clone receiver)
    let rx2 = rx.clone();

    thread::spawn(move || loop {
        if let Err(e) = tx.send(generate_square_matrix()) {
            eprintln!("Producer error sending matrix {}", e);
            break;
        }
        thread::sleep(Duration::from_millis(100));
    });

    while start.elapsed() < Duration::from_secs(3) {
        if let Ok(matrix) = rx.recv() {
            // Parallel sum calculation using Rayon
            let sum = sum_of_matrix_elements(matrix);
            println!("Consumer1: matrix sum = {}", sum);
        }

        if let Ok(matrix) = rx2.recv() {
            // Parallel sum calculation using Rayon
            let sum = sum_of_matrix_elements(matrix);
            println!("Consumer2: matrix sum = {}", sum);
        }
    }

    println!("all we could do in 3 seconds. bye.")
    
}
