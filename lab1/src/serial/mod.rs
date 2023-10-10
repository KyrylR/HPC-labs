use rand::Rng;

mod example;
pub use crate::serial::example::example;

pub fn random_data_initialization(size: i128) -> (Vec<Vec<i128>>, Vec<i128>) {
    let mut rng = rand::thread_rng();

    let vector: Vec<i128> = (0..size)
        .map(|_| (rng.gen_range(0.0..1.0) * i32::MAX as f64) as i128)
        .collect();

    let matrix: Vec<Vec<i128>> = (0..size)
        .map(|_| {
            (0..size)
                .map(|_| (rng.gen_range(0.0..1.0) * i32::MAX as f64) as i128)
                .collect()
        })
        .collect();

    (matrix, vector)
}

pub fn dummy_data_init(size: i128) -> (Vec<Vec<i128>>, Vec<i128>) {
    let mut m: Vec<Vec<i128>> = Vec::new();
    let mut v: Vec<i128> = Vec::new();

    for i in 0..size {
        v.push(1);

        let mut row: Vec<i128> = Vec::new();

        for _ in 0..size {
            row.push(i);
        }

        m.push(row);
    }

    (m, v)
}

pub fn print_matrix(m: &Vec<Vec<i128>>) {
    for row in m {
        print_vector(row)
    }
}

pub fn print_vector(v: &[i128]) {
    for col in v {
        print!("{} ", col);
    }

    println!();
}

pub fn matrix_vector_product(m: &Vec<Vec<i128>>, v: &[i128]) -> Vec<i128> {
    assert_eq!(m.len(), v.len(), "Matrix and vector sizes are not equal");

    m.iter()
        .map(|row| row.iter().enumerate().map(|(i, col)| col * v[i]).sum())
        .collect()
}
