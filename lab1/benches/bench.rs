use criterion::{criterion_group, criterion_main, Criterion};

use lab1::serial::{matrix_vector_product, random_data_initialization};

fn bench_serial_lab1(c: &mut Criterion) {
    let mut group = c.benchmark_group("lab1");

    group.bench_function("random_data_init", |b| {
        b.iter(|| random_data_initialization(500))
    });

    // List of sample sizes for matrix-vector product
    // let sample_sizes = [
    //     10, 100, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
    // ];
    let sample_sizes = [5000]; // comment this line to run all sample sizes and uncomment the line above

    for &size in &sample_sizes {
        let bench_name = format!("matrix_vector_product: {} samples", size);
        let (m, v) = random_data_initialization(size);

        group.bench_function(&bench_name, |b| b.iter(|| matrix_vector_product(&m, &v)));
    }

    group.finish();
}

criterion_group!(benches, bench_serial_lab1);
criterion_main!(benches);
