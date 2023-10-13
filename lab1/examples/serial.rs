use lab1::common::input_size;
use lab1::serial::{
    dummy_data_init, matrix_vector_product, print_matrix, print_vector, random_data_initialization,
};

pub mod parallel;

pub fn main() {
    match input_size() {
        Ok(size) => {
            let (m, v) = random_data_initialization(size);

            print_matrix(&m, size);

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
