use std::collections::HashSet;

const FILE_PATH: &str = "input.txt";

struct Game {
    game_number: u32,
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

impl Game {
    fn new(game_text: &str) -> Self {
        let (game_number_str, cards_string) =
            game_text.split_once(':').expect("Invalid game text format");

        let game_number: u32 = game_number_str
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let winning_numbers: HashSet<u32> = cards_string
            .split('|')
            .next()
            .expect("Failed to split cards_string")
            .split_whitespace()
            .map(|number_str| {
                number_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Failed to parse number_str: {}", number_str))
            })
            .collect();

        let player_numbers: HashSet<u32> = cards_string
            .split('|')
            .nth(1)
            .expect("Failed to split cards_string")
            .split_whitespace()
            .map(|number_str| number_str.trim().parse().unwrap())
            .collect();

        Self {
            game_number,
            winning_numbers,
            player_numbers,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_vector: Vec<Game> = std::fs::read_to_string(FILE_PATH)
        .expect("Failed to read file")
        .lines()
        .map(Game::new)
        .collect();

    let cards_total_worth = game_vector.iter().fold(0, |acc, game| {
        let user_winning_numbers = game.winning_numbers.intersection(&game.player_numbers);
        let user_winning_numbers_count = user_winning_numbers.count();
        if user_winning_numbers_count == 0 {
            acc
        } else {
            acc + (2u32.pow(user_winning_numbers_count as u32 - 1))
        }
    });

    println!("Cards total worth: {}", cards_total_worth);
    Ok(())
}
