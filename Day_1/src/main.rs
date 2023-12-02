use std::fs;

static INPUT_FILE: &str = "input.txt";

fn first_and_last_digit_as_num(string: &str) -> Option<u32> {
    let first_digit = string.chars().find(|char| char.is_numeric())?.to_digit(10)?;
    let last_digit = string.chars().rev().find(|char| char.is_numeric())?.to_digit(10)?;
    Some(first_digit * 10 + last_digit)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_as_string = fs::read_to_string(INPUT_FILE)
        .expect("Something went wrong reading the file");
    let sum = file_as_string.lines().fold(0, |acc, line| {
        acc + first_and_last_digit_as_num(line).unwrap_or(0)
    });
    println!("Sum: {}", sum);
    Ok(())
}