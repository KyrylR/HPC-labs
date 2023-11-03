use rand::Rng;

pub fn dummy_data_initialization(matrix: &mut [f64], vector: &mut [f64], size: usize) {
    for i in 0..size {
        vector[i] = i as f64 + 1.0;
        for j in 0..size {
            matrix[i * size + j] = if j <= i { 1.0 } else { 0.0 };
        }
    }
}

pub fn random_data_initialization(matrix: &mut [f64], vector: &mut [f64], size: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..size {
        vector[i] = rng.gen_range(0.0..100.0f64);
        for j in 0..size {
            matrix[i * size + j] = if j <= i {
                rng.gen_range(0.0..100.0f64)
            } else {
                0.0
            };
        }
    }
}

pub fn print_matrix(matrix: &[f64], row_count: usize, col_count: usize) {
    for row in matrix.chunks(col_count).take(row_count) {
        for &elem in row.iter().take(col_count) {
            print!("{:7.4} ", elem);
        }
        println!();
    }
}

pub fn print_vector(vector: &[f64]) {
    for &elem in vector {
        print!("{:7.4} ", elem);
    }
    println!();
}
