use std::fmt::Display;
use std::io;
use std::str::FromStr;

pub fn input_size<T>() -> Result<T, io::Error>
where
    T: FromStr + Display,
    <T as FromStr>::Err: std::fmt::Debug,
{
    println!("Enter the size of the initial objects: ");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    let size: T = size
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("{:?}", e)))?;

    println!("Chosen objects size = {}", size);

    Ok(size)
}
