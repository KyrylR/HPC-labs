use lab1::common::input_size;

fn process_initialization() -> (Vec<f64>, usize, f64) {
    let eps = 0.0;

    // Input grid size and accuracy
    let size = input_size().unwrap() as usize;

    // Initialize matrix
    let matrix = vec![0.0; size * size];
    (matrix, size, eps)
}

fn dummy_data_initialization(matrix: &mut [f64], size: usize) {
    for i in 0..size {
        for j in 0..size {
            matrix[i * size + j] = if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                100.0
            } else {
                0.0
            };
        }
    }
}

fn result_calculation(matrix: &mut [f64], size: usize, eps: f64) -> usize {
    let mut iterations = 0;
    let mut dmax;

    loop {
        dmax = 0.0;
        for i in 1..(size - 1) {
            for j in 1..(size - 1) {
                let temp = matrix[i * size + j];
                matrix[i * size + j] = 0.25
                    * (matrix[i * size + j + 1]
                        + matrix[i * size + j - 1]
                        + matrix[(i + 1) * size + j]
                        + matrix[(i - 1) * size + j]);
                let dm = (matrix[i * size + j] - temp).abs();
                if dm > dmax {
                    dmax = dm;
                }
            }
        }
        iterations += 1;
        if dmax <= eps {
            break;
        }
    }
    iterations
}

fn main() {
    println!("Serial Gauss-Seidel algorithm in Rust");

    let (mut matrix, size, eps) = process_initialization();
    dummy_data_initialization(&mut matrix, size);

    let t_start = std::time::Instant::now();
    let iterations = result_calculation(&mut matrix, size, eps);
    println!("Time of execution: {:?}", t_start.elapsed());

    println!("Number of iterations: {}", iterations);
}
