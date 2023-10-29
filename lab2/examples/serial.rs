use lab1::common::input_size;
use lab2::serial::{matrix_matrix_product, random_data_initialization};

pub fn main() {
    match input_size() {
        Ok(size) => {
            let (m_a, m_b) = random_data_initialization(size);

            let start = std::time::Instant::now();

            matrix_matrix_product(&m_a, &m_b, size as usize);

            let duration = start.elapsed();

            println!("Time elapsed in matrix_matrix_product() is: {:?}", duration);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
