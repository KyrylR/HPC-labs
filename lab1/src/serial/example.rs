use crate::common::input_size;

use crate::serial::{
    matrix_vector_product, print_matrix, print_vector, random_data_initialization,
};

pub fn example() {
    match input_size() {
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
