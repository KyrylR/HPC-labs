use std::io;
use std::time::Instant;

use lab1::common::input_size;

pub const INFINITIES_PERCENT: f64 = 50.0;
pub const RANDOM_DATA_MULTIPLIER: i32 = 10;

fn min(a: i32, b: i32) -> i32 {
    let result = if a < b { a } else { b };
    if a < 0 && b >= 0 {
        return b;
    }
    if b < 0 && a >= 0 {
        return a;
    }
    if a < 0 && b < 0 {
        return -1;
    }
    result
}

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

pub fn dummy_data_initialization(matrix: &mut [i32], size: usize) {
    for i in 0..size {
        for j in i..size {
            matrix[i * size + j] = if i == j {
                0
            } else if i == 0 {
                j as i32
            } else {
                -1
            };
            matrix[j * size + i] = matrix[i * size + j];
        }
    }
}

pub fn random_data_initialization(matrix: &mut [i32], size: usize) {
    for i in 0..size {
        for j in 0..size {
            if i != j {
                matrix[i * size + j] = if rand::random::<f64>() * 100.0 < INFINITIES_PERCENT {
                    -1
                } else {
                    rand::random::<i32>() % RANDOM_DATA_MULTIPLIER
                };
            } else {
                matrix[i * size + j] = 0;
            }
        }
    }
}

fn serial_floyd(matrix: &mut [i32], size: usize) {
    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                if matrix[i * size + k] != -1 && matrix[k * size + j] != -1 {
                    let t1 = matrix[i * size + j];
                    let t2 = matrix[i * size + k] + matrix[k * size + j];
                    matrix[i * size + j] = min(t1, t2);
                }
            }
        }
    }
}

fn print_matrix(matrix: &[i32], row_count: usize, col_count: usize) {
    for i in 0..row_count {
        for j in 0..col_count {
            print!("{:7}", matrix[i * col_count + j]);
        }
        println!();
    }
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
