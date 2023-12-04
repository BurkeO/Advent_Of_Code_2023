use std::error::Error;

const FILE_PATH: &str = "input.txt";

// enum SchematicType {
//     Period,
//     Number(u32),
//     Symbol(char),
// }

struct Schematic {
    schematic_matrix: Vec<Vec<char>>,
}

impl Schematic {
    fn new(schematic_file: &std::path::Path) -> Self {
        let mut schematic_matrix: Vec<Vec<char>> = vec![vec![]];
        std::fs::read_to_string(schematic_file)
            .expect("Something went wrong reading the file")
            .lines()
            .for_each(|line| {
                let mut row: Vec<char> = vec![];
                line.chars().for_each(|c| row.push(c));
                schematic_matrix.push(row);
            });
        Self { schematic_matrix }
    }

    fn sum_part_numbers(&self) -> u32 {
        let mut sum: u32 = 0;
        for row in 0..self.schematic_matrix.len() {
            let mut col = 0;
            while col < self.schematic_matrix[row].len() {
                if self.schematic_matrix[row][col].is_digit(10) {
                    let number = self.schematic_matrix[row]
                        .iter()
                        .skip(col)
                        .take_while(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    println!("Found number: {}", number);
                    col += number.to_string().len();


                    //todo is adjacent to symbol

                } else {
                    col += 1;
                }
            }
        }
        sum
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let schematic = Schematic::new(std::path::Path::new(FILE_PATH));
    println!("Sum of part numbers: {}", schematic.sum_part_numbers());
    Ok(())
}
