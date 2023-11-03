use std::io;
use std::time::{Duration, Instant};

const RANDOM_DATA_MULTIPLIER: f64 = 1000.0;

fn main() {
    println!("Serial bubble sort program");

    // Process initialization
    let mut data = process_initialization();

    println!("Data before sorting");
    print_data(&data);

    // Serial bubble sort
    let start = Instant::now();
    serial_bubble_sort(&mut data);
    let duration = start.elapsed();

    println!("Data after sorting");
    print_data(&data);

    println!("Time of execution: {:?}", duration);
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

// Function for simple setting the initial data
fn dummy_data_initialization(data_size: usize) -> Vec<f64> {
    (1..=data_size).rev().map(|i| i as f64).collect()
}

// Function for initializing the data by the random generator
fn random_data_initialization(data_size: usize) -> Vec<f64> {
    let mut data = vec![0.0; data_size];
    let mut rng = rand::thread_rng();
    for element in &mut data {
        *element = rand::random::<f64>() * RANDOM_DATA_MULTIPLIER;
    }
    data
}

// Function for the serial bubble sort algorithm
fn serial_bubble_sort(data: &mut [f64]) {
    let mut tmp;
    let data_size = data.len();
    for i in 1..data_size {
        for j in 0..data_size - i {
            if data[j] > data[j + 1] {
                tmp = data[j];
                data[j] = data[j + 1];
                data[j + 1] = tmp;
            }
        }
    }
}

// Function for formatted data output
fn print_data(data: &[f64]) {
    for &value in data {
        print!("{:7.4} ", value);
    }
    println!();
}

// Sorting by the standard library algorithm
fn serial_std_sort(data: &mut [f64]) {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
}
