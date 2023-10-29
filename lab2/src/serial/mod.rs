use rand::Rng;

pub fn matrix_matrix_product(m_a: &[u64], m_b: &[u64], size: usize) -> Vec<u128> {
    (0..size)
        .flat_map(|i| {
            (0..size).map(move |j| {
                (0..size)
                    .map(|k| m_a[i * size + k] as u128 * m_b[k * size + j] as u128)
                    .sum()
            })
        })
        .collect()
}

pub fn random_data_initialization(size: u64) -> (Vec<u64>, Vec<u64>) {
    let mut rng = rand::thread_rng();

    let vector: Vec<u64> = (0..size * size)
        .map(|_| rng.gen_range(0_u64..u16::MAX as u64))
        .collect();
    let matrix: Vec<u64> = (0..size * size)
        .map(|_| rng.gen_range(0_u64..u16::MAX as u64))
        .collect();

    (matrix, vector)
}

pub fn dummy_data_init(size: u64) -> (Vec<u64>, Vec<u64>) {
    let m_a = (1..size + 1)
        .flat_map(|i| std::iter::repeat(i).take(size as usize))
        .collect();

    let m_b = (1..size + 1)
        .flat_map(|i| std::iter::repeat(i).take(size as usize))
        .collect();

    (m_a, m_b)
}
