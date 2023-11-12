use std::io;
use std::time::Instant;

use lab1::common::input_size;
use lab5::serial::{dummy_data_initialization, serial_floyd};

fn process_initialization() -> Result<(usize, Vec<i32>), io::Error> {
    let mut size: usize = 0;
    while size == 0 {
        size = input_size()? as usize;

        if size == 0 {
            println!("The number of vertices should be greater than zero");
        }
    }

    let mut matrix = vec![0; size * size];
    dummy_data_initialization(&mut matrix, size);
    // random_data_initialization(&mut matrix, size); // Uncomment if needed

    Ok((size, matrix))
}

fn main() -> Result<(), io::Error> {
    println!("Serial Floyd algorithm");
    let (size, mut matrix) = process_initialization()?;

    let start = Instant::now();
    serial_floyd(&mut matrix, size);
    let duration = start.elapsed();

    println!("Time of execution: {:?}", duration);

    Ok(())
}
