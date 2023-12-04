use std::error::Error;

const FILE_PATH: &str = "input.txt";

// enum SchematicType {
//     Period,
//     Number(u32),
//     Symbol(char),
// }

struct Schematic {
    schematic_matrix: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Schematic {
    fn new(schematic_file: &std::path::Path) -> Self {
        let mut schematic_matrix: Vec<Vec<char>> = vec![vec![]];
        schematic_matrix.clear();
        std::fs::read_to_string(schematic_file)
            .expect("Something went wrong reading the file")
            .lines()
            .for_each(|line| {
                let mut row: Vec<char> = vec![];
                line.chars().for_each(|c| row.push(c));
                schematic_matrix.push(row);
            });
        Self {
            height: schematic_matrix.len(),
            width: schematic_matrix[0].len(),
            schematic_matrix,
        }
    }

    fn sum_part_numbers(&self) -> u32 {
        let mut sum: u32 = 0;
        for row in 0..self.height {
            let mut col = 0;
            while col < self.width {
                if self.schematic_matrix[row][col].is_ascii_digit() {
                    let number = self.schematic_matrix[row]
                        .iter()
                        .skip(col)
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    println!("Found number: {}", number);
                    let number_length = number.to_string().len();

                    for number_col in col..col + number_length {
                        if self.is_adjacent_to_symbol(row, number_col) {
                            sum += number;
                            break;
                        }
                    }
                    col += number_length;
                } else {
                    col += 1;
                }
            }
        }
        sum
    }

    fn is_adjacent_to_symbol(&self, row: usize, col: usize) -> bool {
        let start_row = if row > 0 { row - 1 } else { row };
        let end_row = if row < self.height - 1 { row + 1 } else { row };
        let start_col = if col > 0 { col - 1 } else { col };
        let end_col = if col < self.width - 1 { col + 1 } else { col };
        for r in start_row..=end_row {
            for c in start_col..=end_col {
                if self.schematic_matrix[r][c] != '.'
                    && self.schematic_matrix[r][c].is_ascii_digit() == false
                {
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let schematic = Schematic::new(std::path::Path::new(FILE_PATH));
    println!("Sum of part numbers: {}", schematic.sum_part_numbers());
    Ok(())
}
