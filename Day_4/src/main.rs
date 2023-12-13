use std::collections::HashSet;

const FILE_PATH: &str = "input.txt";

struct Game {
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

fn parse_numbers(string_of_numbers: &str) -> HashSet<u32> {
    string_of_numbers
        .split_whitespace()
        .map(|number| number.trim().parse().expect("Failed to parse number"))
        .collect()
}

impl Game {
    fn new(game_text: &str) -> Self {
        let (_, cards_string) = game_text.split_once(':').expect("Invalid game text format");

        let winning_numbers: HashSet<u32> = parse_numbers(
            cards_string
                .split('|')
                .next()
                .expect("Failed to split cards_string"),
        );

        let player_numbers: HashSet<u32> = parse_numbers(
            cards_string
                .split('|')
                .nth(1)
                .expect("Failed to split cards_string"),
        );

        Self {
            winning_numbers,
            player_numbers,
        }
    }
}

fn count_games(games: &[Game], game_index: usize, current_count: &mut u64) {
    if game_index < games.len() {
        *current_count += 1;
        let winning_games_count = games[game_index]
            .winning_numbers
            .intersection(&games[game_index].player_numbers)
            .count();
        if winning_games_count == 0 {
            return;
        }
        let length_to_take = if games.len() < winning_games_count {
            games.len()
        } else {
            winning_games_count + 1
        };
        for offset in 1..length_to_take {
            count_games(games, game_index + offset, current_count)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_vector: Vec<Game> = std::fs::read_to_string(FILE_PATH)
        .expect("Failed to read file")
        .lines()
        .map(Game::new)
        .collect();

    let mut cards_count = 0;
    for start_game_index in 0..game_vector.len() {
        count_games(&game_vector, start_game_index, &mut cards_count);
    }
    println!("Cards Count {}", cards_count);

    let cards_total_worth = game_vector.iter().fold(0, |total_worth, game| {
        let user_winning_numbers = game.winning_numbers.intersection(&game.player_numbers);
        let user_winning_numbers_count = user_winning_numbers.count();
        if user_winning_numbers_count == 0 {
            total_worth
        } else {
            total_worth + (2u32.pow(user_winning_numbers_count as u32 - 1))
        }
    });

    println!("Cards total worth: {}", cards_total_worth);
    Ok(())
}
