use std::io;

use lab1::{matrix_vector_product, print_matrix, print_vector, random_data_initialization};

fn main() {
    match process_init() {
        Ok(size) => {
            let (m, v) = random_data_initialization(size);

            println!("Matrix:");
            print_matrix(&m);

            println!("Vector:");
            print_vector(&v);

            let start = std::time::Instant::now();

            matrix_vector_product(&m, &v);

            let duration = start.elapsed();

            println!("Time elapsed in matrix_vector_product() is: {:?}", duration);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn process_init() -> Result<i128, io::Error> {
    println!("Enter the size of the initial objects: ");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    let size: i128 = size
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    println!("Chosen objects size = {}", size);

    Ok(size)
}
