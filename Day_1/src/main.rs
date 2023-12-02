use phf::phf_map;
use std::{fs, str::Chars};

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

fn find_pos_and_value_of_digit_char(iter: Chars) -> Option<(usize, u32)> {
    iter.enumerate()
        .find(|(_, char)| char.is_numeric())
        .map(|(index, char)| (index, char.to_digit(10).unwrap()))
}

fn first_and_last_digit_as_num(input_string: &str) -> Option<u32> {
    let first_digit_position_value_opt = find_pos_and_value_of_digit_char(input_string.chars());
    let mut last_digit_position_value_opt =
        find_pos_and_value_of_digit_char(input_string.chars().rev().collect::<String>().chars());
    
    if last_digit_position_value_opt.is_some() {
        last_digit_position_value_opt = Some((
            input_string.len() - 1 - last_digit_position_value_opt.unwrap().0,
            last_digit_position_value_opt.unwrap().1,
        ));
    }

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
            else {
                first_digit_string_position_value_opt = Some((position, digit_int))
            }
            if let Some((last_digit_string_pos, _)) = last_digit_string_position_value_opt {
                last_digit_string_position_value_opt = if position > last_digit_string_pos {
                    Some((position, digit_int))
                } else {
                    last_digit_string_position_value_opt
                }
            }
            else {
                last_digit_string_position_value_opt = Some((position, digit_int))
            }
        }
    }

    let mut first_digit_opt: Option<u32> = None;
    let mut last_digit_opt: Option<u32> = None;

    match (
        first_digit_position_value_opt,
        first_digit_string_position_value_opt,
    ) {
        (Some((digit_pos, digit)), Some((digit_string_pos, digit_int))) => {
            if digit_pos < digit_string_pos {
                first_digit_opt = Some(digit);
            } else {
                first_digit_opt = Some(*digit_int);
            }
        }
        (Some((_, digit)), None) => {
            first_digit_opt = Some(digit);
        }
        (None, Some((_, digit))) => {
            first_digit_opt = Some(*digit);
        }
        (None, None) => return None,
    }

    match (
        last_digit_position_value_opt,
        last_digit_string_position_value_opt,
    ) {
        (Some((digit_pos, digit)), Some((digit_string_pos, digit_int))) => {
            if digit_pos > digit_string_pos {
                last_digit_opt = Some(digit);
            } else {
                last_digit_opt = Some(*digit_int);
            }
        }
        (Some((_, digit)), None) => {
            last_digit_opt = Some(digit);
        }
        (None, Some((_, digit))) => {
            last_digit_opt = Some(*digit);
        }
        (None, None) => return None,
    }


    let val = first_digit_opt.map(|first_digit| first_digit * 10 + last_digit_opt.unwrap());
    println!("{}", val.unwrap());
    val
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_as_string =
        fs::read_to_string(INPUT_FILE).expect("Something went wrong reading the file");
    let sum = file_as_string.lines().fold(0, |acc, line| {
        acc + first_and_last_digit_as_num(line).unwrap()
    });
    println!("Sum: {}", sum);
    Ok(())
}
