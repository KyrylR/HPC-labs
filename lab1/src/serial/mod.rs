use rand::Rng;

macro_rules! impl_bounded {
    ($($t:ty),+) => {
        $(
            impl Bounded for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
            }
        )+
    };
}

// Define our custom trait
pub trait Bounded {
    const MIN: Self;
    const MAX: Self;
}

impl_bounded!(i32, i64, i128);

pub fn random_data_initialization<T>(size: usize) -> (Vec<Vec<T>>, Vec<T>)
where
    T: rand::distributions::uniform::SampleUniform + Bounded + PartialOrd + Copy,
{
    let mut rng = rand::thread_rng();

    let vector: Vec<T> = (0..size).map(|_| rng.gen_range(T::MIN..T::MAX)).collect();

    let matrix: Vec<Vec<T>> = (0..size)
        .map(|_| (0..size).map(|_| rng.gen_range(T::MIN..T::MAX)).collect())
        .collect();

    (matrix, vector)
}

pub fn dummy_data_init<T>(size: usize) -> (Vec<Vec<T>>, Vec<T>)
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + From<u8>,
{
    let mut m: Vec<Vec<T>> = Vec::with_capacity(size);
    let v = vec![T::from(1); size];

    for i in 0..size {
        m.push(vec![T::from(i as u8); size]);
    }

    (m, v)
}

pub fn print_matrix<T: std::fmt::Display>(m: &[Vec<T>]) {
    println!("Matrix:");

    for row in m {
        print_vector(row)
    }
}

pub fn print_vector<T: std::fmt::Display>(v: &[T]) {
    print!("Vector: ");

    for col in v {
        print!("{} ", col);
    }
    println!();
}

pub fn matrix_vector_product<T, R>(m: &[Vec<T>], v: &[T]) -> Vec<R>
where
    T: Copy,
    R: std::ops::Mul<Output = R> + std::ops::Add<Output = R> + Copy + std::iter::Sum + From<T>,
{
    assert_eq!(m.len(), v.len(), "Matrix and vector sizes are not equal");

    m.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, col)| R::from(*col) * R::from(v[i]))
                .sum()
        })
        .collect()
}
