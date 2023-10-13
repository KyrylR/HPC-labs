use lab1::common::input_size;
use lab1::serial::{matrix_vector_product, random_data_initialization};

pub mod parallel;

pub fn main() {
    match input_size() {
        Ok(size) => {
            let (m, v) = random_data_initialization(size);

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
