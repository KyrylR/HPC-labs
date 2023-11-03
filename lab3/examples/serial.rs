use std::io;
use std::time::Instant;

use lab3::serial::{print_matrix, print_vector, random_data_initialization};

fn process_initialization(size: usize) -> (Vec<f64>, Vec<f64>) {
    let mut matrix = vec![0.0; size * size];
    let mut vector = vec![0.0; size];

    // Initialization of the matrix and the vector elements
    random_data_initialization(&mut matrix, &mut vector, size);

    (matrix, vector)
}

fn find_pivot_row(matrix: &[f64], size: usize, iter: usize, serial_pivot_iter: &[i32]) -> i32 {
    let mut pivot_row = -1; // Index of the pivot row
    let mut max_value = 0.0; // Value of the pivot element

    for i in 0..size {
        if serial_pivot_iter[i] == -1 && matrix[i * size + iter].abs() > max_value {
            pivot_row = i as i32;
            max_value = matrix[i * size + iter].abs();
        }
    }

    pivot_row
}

fn serial_column_elimination(
    matrix: &mut [f64],
    vector: &mut [f64],
    pivot: usize,
    iter: usize,
    size: usize,
    serial_pivot_iter: &[i32],
) {
    let pivot_value = matrix[pivot * size + iter];
    for i in 0..size {
        if serial_pivot_iter[i] == -1 {
            let pivot_factor = matrix[i * size + iter] / pivot_value;
            for j in iter..size {
                matrix[i * size + j] -= pivot_factor * matrix[pivot * size + j];
            }
            vector[i] -= pivot_factor * vector[pivot];
        }
    }
}

fn serial_gaussian_elimination(
    matrix: &mut [f64],
    vector: &mut [f64],
    size: usize,
    serial_pivot_pos: &mut [i32],
    serial_pivot_iter: &mut [i32],
) {
    for iter in 0..size {
        let pivot_row = find_pivot_row(matrix, size, iter, serial_pivot_iter) as usize;
        serial_pivot_pos[iter] = pivot_row as i32;
        serial_pivot_iter[pivot_row] = iter as i32;
        serial_column_elimination(matrix, vector, pivot_row, iter, size, serial_pivot_iter);
    }
}

fn serial_back_substitution(
    matrix: &[f64],
    vector: &mut [f64],
    result: &mut [f64],
    size: usize,
    serial_pivot_pos: &[i32],
) {
    for i in (0..size).rev() {
        let row_index = serial_pivot_pos[i] as usize;
        result[i] = vector[row_index] / matrix[size * row_index + i];
        for j in 0..i {
            let row = serial_pivot_pos[j] as usize;
            vector[j] -= matrix[row * size + i] * result[i];
        }
    }
}

fn serial_result_calculation(matrix: &mut [f64], vector: &mut [f64], size: usize) -> Vec<f64> {
    let mut result = vec![0.0; size];
    let mut serial_pivot_pos = vec![-1; size];
    let mut serial_pivot_iter = vec![-1; size];

    // Gaussian elimination
    serial_gaussian_elimination(
        matrix,
        vector,
        size,
        &mut serial_pivot_pos,
        &mut serial_pivot_iter,
    );

    // Back substitution
    serial_back_substitution(matrix, vector, &mut result, size, &serial_pivot_pos);

    result
}

fn main() {
    // Set the size of the matrix and the vector
    println!("Serial Gauss algorithm for solving linear systems");
    println!("Enter the size of the matrix and the vector: ");

    let mut size_str = String::new();
    io::stdin()
        .read_line(&mut size_str)
        .expect("Failed to read line");
    let size: usize = size_str.trim().parse().expect("Please type a number!");

    // Memory allocation and definition of objects' elements
    let (mut matrix, mut vector) = process_initialization(size);

    // The matrix and the vector output
    println!("Initial Matrix");
    print_matrix(&matrix, size, size);
    println!("Initial Vector");
    print_vector(&vector);

    // Execution of the Gauss algorithm
    let start = Instant::now();
    let result = serial_result_calculation(&mut matrix, &mut vector, size);
    let duration = start.elapsed();

    // Printing the result vector
    println!("\nResult Vector:");
    print_vector(&result);

    // Printing the execution time of the Gauss method
    println!("\nTime of execution: {:?}", duration);
}
