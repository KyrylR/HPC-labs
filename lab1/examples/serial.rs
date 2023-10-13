use lab1::common::input_size;
use lab1::serial::{matrix_vector_product, print_matrix, print_vector, random_data_initialization};

pub mod parallel;

pub fn main() {
    match input_size() {
        Ok(size) => {
            let (m, v) = random_data_initialization::<i64>(size);

            print_matrix(&m);
            print_vector(&v);

            let start = std::time::Instant::now();

            matrix_vector_product::<i64, i128>(&m, &v);

            let duration = start.elapsed();

            println!("Time elapsed in matrix_vector_product() is: {:?}", duration);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
