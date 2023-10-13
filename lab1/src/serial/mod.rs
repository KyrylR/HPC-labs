use rand::Rng;

pub fn random_data_initialization(size: u64) -> (Vec<u64>, Vec<u64>) {
    let mut rng = rand::thread_rng();

    let vector: Vec<u64> = (0..size)
        .map(|_| rng.gen_range(0_u64..u32::MAX as u64))
        .collect();
    let matrix: Vec<u64> = (0..size * size)
        .map(|_| rng.gen_range(0_u64..u32::MAX as u64))
        .collect();

    (matrix, vector)
}

pub fn dummy_data_init(size: u64) -> (Vec<u64>, Vec<u64>) {
    let m = (0..size)
        .flat_map(|i| std::iter::repeat(i).take(size as usize))
        .collect();

    let v = vec![1; size as usize];

    (m, v)
}

pub fn print_matrix(m: &[u64], size: u64) {
    println!("Matrix:");

    m.chunks(size as usize)
        .for_each(|row| print_vector(&row.to_vec()));
}

pub fn print_vector(v: &Vec<u64>) {
    for col in v {
        print!("{} ", col);
    }
    println!();
}

pub fn matrix_vector_product(m: &Vec<u64>, v: &Vec<u64>) -> Vec<u64> {
    assert_eq!(
        m.len(),
        v.len() * v.len(),
        "Matrix and vector sizes are not equal"
    );

    m.chunks(v.len())
        .map(|row| row.iter().zip(v.iter()).map(|(col, v)| col * v).sum())
        .collect()
}
