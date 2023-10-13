use std::io;

pub fn input_size() -> Result<u64, io::Error> {
    println!("Enter the size of the initial objects: ");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    let size: u64 = size
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("{:?}", e)))?;

    println!("Chosen objects size = {}", size);

    Ok(size)
}
