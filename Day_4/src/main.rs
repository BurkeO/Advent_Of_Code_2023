use std::collections::HashSet;

const FILE_PATH: &str = "input.txt";

struct Game {
    game_number: u32,
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

impl Game {
    fn new(game_text: &str) -> Self {
        let game_number = game_text
            .split(':')
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();
        let cards_string = game_text.split(':').nth(1).unwrap().to_string();
        let winning_numbers: HashSet<u32> = cards_string
            .split('|')
            .nth(0)
            .expect("failed to take first split of cards_string")
            .split_whitespace()
            .map(|number_str| {
                number_str
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("failed to parse number_str: {}", number_str))
            })
            .collect();
        let player_numbers = cards_string
            .split('|')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|number_str| number_str.trim().parse::<u32>().unwrap())
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
        .expect("failed_to_read_file")
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
