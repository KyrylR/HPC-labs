use rand::Rng;

mod example;
pub use crate::serial::example::example;

pub fn random_data_initialization(size: i64) -> (Vec<Vec<i64>>, Vec<i64>) {
    let mut rng = rand::thread_rng();

    let vector: Vec<i64> = (0..size)
        .map(|_| (rng.gen_range(0.0..1.0) * i32::MAX as f64) as i64)
        .collect();

    let matrix: Vec<Vec<i64>> = (0..size)
        .map(|_| {
            (0..size)
                .map(|_| (rng.gen_range(0.0..1.0) * i32::MAX as f64) as i64)
                .collect()
        })
        .collect();

    (matrix, vector)
}

pub fn dummy_data_init(size: i64) -> (Vec<Vec<i64>>, Vec<i64>) {
    let mut m: Vec<Vec<i64>> = Vec::new();
    let mut v: Vec<i64> = Vec::new();

    for i in 0..size {
        v.push(1);

        let mut row: Vec<i64> = Vec::new();

        for _ in 0..size {
            row.push(i);
        }

        m.push(row);
    }

    (m, v)
}

pub fn print_matrix(m: &Vec<Vec<i64>>) {
    for row in m {
        print_vector(row)
    }
}

pub fn print_vector(v: &[i64]) {
    for col in v {
        print!("{} ", col);
    }

    println!();
}

pub fn matrix_vector_product(m: &Vec<Vec<i64>>, v: &[i64]) -> Vec<i64> {
    assert_eq!(m.len(), v.len(), "Matrix and vector sizes are not equal");

    m.iter()
        .map(|row| row.iter().enumerate().map(|(i, col)| col * v[i]).sum())
        .collect()
}
