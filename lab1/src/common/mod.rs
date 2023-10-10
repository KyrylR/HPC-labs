use std::io;

pub fn process_init() -> Result<i128, io::Error> {
    println!("Enter the size of the initial objects: ");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    let size: i128 = size
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    println!("Chosen objects size = {}", size);

    Ok(size)
}
