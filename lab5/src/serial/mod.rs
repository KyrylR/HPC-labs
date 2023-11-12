use std::cmp;

pub const INFINITIES_PERCENT: f64 = 50.0;
pub const RANDOM_DATA_MULTIPLIER: i32 = 10;

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

pub fn print_matrix(matrix: &[i32], row_count: usize, col_count: usize) {
    for i in 0..row_count {
        for j in 0..col_count {
            print!("{:7}", matrix[i * col_count + j]);
        }
        println!();
    }
}

pub fn min(a: i32, b: i32) -> i32 {
    match (a, b) {
        (a, b) if a < 0 && b >= 0 => b,
        (a, b) if b < 0 && a >= 0 => a,
        (a, b) if a < 0 && b < 0 => -1,
        _ => cmp::min(a, b),
    }
}

pub fn serial_floyd(matrix: &mut [i32], size: usize) {
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
