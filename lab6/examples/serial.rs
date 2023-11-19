use lab1::common::input_size;
use lab6::serial::lab6_main;

fn main() {
    println!("Serial Gauss-Seidel algorithm in Rust");

    // Input grid size and accuracy
    let size = input_size().unwrap() as usize;

    lab6_main(size);
}
