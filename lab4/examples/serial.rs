use std::io;
use std::time::Instant;

const RANDOM_DATA_MULTIPLIER: f64 = 1000.0;

fn main() {
    println!("Serial bubble sort program");

    // Process initialization
    let mut data = process_initialization();

    // Serial bubble sort
    let start = Instant::now();
    serial_bubble_sort(&mut data);
    let duration = start.elapsed();
    println!(
        "Time of execution of custom serial bubble sort: {:?}",
        duration
    );

    // Serial std sort
    let start = Instant::now();
    serial_std_sort(&mut data);
    let duration = start.elapsed();
    println!("Time of execution of serial std sort: {:?}", duration);
}

// Function for allocating the memory and setting the initial values
fn process_initialization() -> Vec<f64> {
    let mut data_size = 0;
    while data_size == 0 {
        println!("Enter the size of data to be sorted: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        data_size = input.trim().parse().unwrap_or(0);
        if data_size == 0 {
            println!("Data size should be greater than zero");
        }
    }
    println!("Sorting {} data items", data_size);

    // Simple setting the data
    // dummy_data_initialization(data_size)

    // Setting the data by the random generator
    random_data_initialization(data_size)
}

pub fn dummy_data_initialization(data_size: usize) -> Vec<f64> {
    (1..=data_size).rev().map(|i| i as f64).collect()
}

fn random_data_initialization(data_size: usize) -> Vec<f64> {
    let mut data = vec![0.0; data_size];
    for element in &mut data {
        *element = rand::random::<f64>() * RANDOM_DATA_MULTIPLIER;
    }
    data
}

fn serial_bubble_sort(data: &mut [f64]) {
    for i in 0..data.len() {
        for j in 0..data.len() - 1 - i {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
        }
    }
}

pub fn print_data(data: &[f64]) {
    for &value in data {
        print!("{:7.4} ", value);
    }
    println!();
}

pub fn serial_std_sort(data: &mut [f64]) {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
}
