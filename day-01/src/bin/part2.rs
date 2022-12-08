use day_01::process_part2;
use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day-01/input.txt")?;
    let result = process_part1(&input);
    println!("{:?}", result);
    Ok(())
}