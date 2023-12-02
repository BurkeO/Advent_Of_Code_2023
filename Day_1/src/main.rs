use phf::phf_map;
use std::fs;

static INPUT_FILE: &str = "input.txt";

static DIGIT_STRINGS_TO_INT_MAP: phf::Map<&str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};

fn first_and_last_digit_as_num(input_string: &str) -> Option<u32> {
    let first_digit_position_opt = input_string.chars().position(|char| char.is_numeric());
    let last_digit_position_opt = input_string
        .chars()
        .rev()
        .position(|char| char.is_numeric());

    let mut first_digit_string_position_value_opt: Option<(usize, &u32)> = None;
    let mut last_digit_string_position_value_opt: Option<(usize, &u32)> = None;

    for (digit_string, digit_int) in DIGIT_STRINGS_TO_INT_MAP.into_iter() {
        if let Some(position) = input_string.find(digit_string) {
            if let Some((first_digit_string_pos, _)) = first_digit_string_position_value_opt {
                first_digit_string_position_value_opt = if position < first_digit_string_pos {
                    Some((position, digit_int))
                } else {
                    first_digit_string_position_value_opt
                }
            }
            if let Some((last_digit_string_pos, _)) = last_digit_string_position_value_opt {
                last_digit_string_position_value_opt = if position > last_digit_string_pos {
                    Some((position, digit_int))
                } else {
                    last_digit_string_position_value_opt
                }
            }
        }
    }

    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    match (
        first_digit_position_opt,
        first_digit_string_position_value_opt,
    ) {
        (Some(digit_pos), Some((digit_string_pos, digit_int))) => {
            if digit_pos < digit_string_pos {
                first_digit = Some(digit_pos as u32);
            } else {
                first_digit = Some(*digit_int);
            }
        }
        (Some(first_digit_position), None) => {
            first_digit = Some(first_digit_position as u32);
        }
        (None, Some((_, first_digit_int))) => {
            first_digit = Some(*first_digit_int);
        }
        (None, None) => return None,
    }

    Some(1 * 10 + 2)
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_as_string =
        fs::read_to_string(INPUT_FILE).expect("Something went wrong reading the file");
    let sum = file_as_string.lines().fold(0, |acc, line| {
        acc + first_and_last_digit_as_num(line).unwrap_or(0)
    });
    println!("Sum: {}", sum);
    Ok(())
}
