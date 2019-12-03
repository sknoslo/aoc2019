use std::io;
use std::io::Read;

pub mod computer;

pub fn get_puzzle_input() -> io::Result<String> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;
    let input = input.trim().to_string();

    Ok(input)
}
