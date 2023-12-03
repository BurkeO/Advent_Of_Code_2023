const INPUT_PATH: &str = "input.txt";

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

#[derive(Debug)]
struct GameReveal {
    blue: u32,
    red: u32,
    green: u32,
}

impl GameReveal {
    fn new(reveal_line: &str) -> Self {
        let mut blue_count = 0;
        let mut red_count = 0;
        let mut green_count = 0;
        reveal_line.split(',').for_each(|count_colour_str| {
            let trimmed_count_colour_str = count_colour_str.trim();
            let colour = trimmed_count_colour_str.split(' ').nth(1).unwrap();
            let count = trimmed_count_colour_str
                .split(' ')
                .nth(0)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            if colour.to_lowercase() == "red" {
                red_count = count;
            } else if colour.to_lowercase() == "blue" {
                blue_count = count;
            } else if colour.to_lowercase() == "green" {
                green_count = count;
            }
        });

        Self {
            blue: blue_count,
            red: red_count,
            green: green_count,
        }
    }
}

struct Game {
    id: u32,
    reveals: Vec<GameReveal>,
}

impl Game {
    fn new(game_line: String) -> Self {
        let id = game_line[5..]
            .chars()
            .take_while(|char| *char != ':')
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let reveals: Vec<GameReveal> = game_line
            .split(':')
            .nth(1)
            .unwrap()
            .split(';')
            .map(GameReveal::new)
            .collect();

        Self { id, reveals }
    }

    fn is_possible(&self) -> bool {
        for reveal in self.reveals.iter() {
            if reveal.red > MAX_RED_CUBES
                || reveal.blue > MAX_BLUE_CUBES
                || reveal.green > MAX_GREEN_CUBES
            {
                return false;
            }
        }
        true
    }

    fn get_max_blue(&self) -> u32 {
        self.reveals
            .iter()
            .map(|reveal| reveal.blue)
            .max()
            .unwrap_or(0)
    }

    fn get_max_red(&self) -> u32 {
        self.reveals
            .iter()
            .map(|reveal| reveal.red)
            .max()
            .unwrap_or(0)
    }

    fn get_max_green(&self) -> u32 {
        self.reveals
            .iter()
            .map(|reveal| reveal.green)
            .max()
            .unwrap_or(0)
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let id_sum =
    let power_sum = std::fs::read_to_string(INPUT_PATH)
        .expect("Error reading input file!")
        .lines()
        // .fold(0, |game_id_sum, line| {
        .fold(0, |cubes_power_sum, line| {
            let game = Game::new(line.to_string());
            let max_blues = game.get_max_blue();
            let max_greens = game.get_max_green();
            let max_reds = game.get_max_red();
            cubes_power_sum + (max_blues * max_greens * max_reds)
        });
            
            // if game.is_possible() {
            //     game_id_sum + game.id
            // } else {
            //     game_id_sum
            // }
        
    // print!("Sum of possible game IDs: {}", id_sum);
    print!("Sum of cube powers: {}", power_sum);
    Ok(())
}
