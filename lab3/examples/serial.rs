use lab3::serial::lab3_main;
use std::io;

fn main() {
    // Set the size of the matrix and the vector
    println!("Serial Gauss algorithm for solving linear systems");
    println!("Enter the size of the matrix and the vector: ");

    let mut size_str = String::new();
    io::stdin()
        .read_line(&mut size_str)
        .expect("Failed to read line");
    let size: usize = size_str.trim().parse().expect("Please type a number!");

    lab3_main(size);
}
