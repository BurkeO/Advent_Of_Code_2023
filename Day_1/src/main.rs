use phf::phf_map;
use std::fs;

static INPUT_FILE: &str = "input.txt";

static DIGIT_STRINGS_TO_INT_MAP: phf::Map<&str, u32> = phf_map! {
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
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
    let mut first_digit_string_position_value_opt: Option<(usize, &u32)> = None;
    let mut last_digit_string_position_value_opt: Option<(usize, &u32)> = None;

    for (digit_string, digit_int) in DIGIT_STRINGS_TO_INT_MAP.into_iter() {
        let first_position = input_string.find(digit_string);
        let last_position = input_string.rfind(digit_string);

        if let Some(first_pos) = first_position {
            if let Some((first_digit_string_pos, _)) = first_digit_string_position_value_opt {
                first_digit_string_position_value_opt = if first_pos < first_digit_string_pos {
                    Some((first_pos, digit_int))
                } else {
                    first_digit_string_position_value_opt
                }
            } else {
                first_digit_string_position_value_opt = Some((first_pos, digit_int))
            }
        }

        if let Some(last_pos) = last_position {
            if let Some((last_digit_string_pos, _)) = last_digit_string_position_value_opt {
                last_digit_string_position_value_opt = if last_pos > last_digit_string_pos {
                    Some((last_pos, digit_int))
                } else {
                    last_digit_string_position_value_opt
                }
            } else {
                last_digit_string_position_value_opt = Some((last_pos, digit_int))
            }
        }
    }

    let val = first_digit_string_position_value_opt.unwrap().1 * 10
        + last_digit_string_position_value_opt.unwrap().1;
    Some(val)
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
