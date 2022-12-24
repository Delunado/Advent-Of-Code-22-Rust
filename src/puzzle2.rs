use std::fs;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

enum PlayerDataManagementType {
    AsMovement,
    AsGuide,
}

#[derive(PartialEq, Eq, Ord)]
enum Movement {
    //A, X
    Rock,

    //B, Y
    Paper,

    //C, Z
    Scissor,
}

impl PartialOrd for Movement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Movement::Rock, Movement::Rock) => Some(Equal),
            (Movement::Paper, Movement::Paper) => Some(Equal),
            (Movement::Scissor, Movement::Scissor) => Some(Equal),

            (Movement::Rock, Movement::Paper) => Some(Less),
            (Movement::Rock, Movement::Scissor) => Some(Greater),

            (Movement::Paper, Movement::Rock) => Some(Greater),
            (Movement::Paper, Movement::Scissor) => Some(Less),

            (Movement::Scissor, Movement::Rock) => Some(Less),
            (Movement::Scissor, Movement::Paper) => Some(Greater)
        }
    }
}

struct Strategy {
    enemy_movement: Movement,
    player_movement: Movement,
}

impl Strategy {
    pub fn new(enemy_movement_data: &str, player_movement_data: &str, player_data_management_type: PlayerDataManagementType) -> Strategy {
        let enemy_movement = match enemy_movement_data {
            "A" => Movement::Rock,
            "B" => Movement::Paper,
            "C" => Movement::Scissor,
            _ => Movement::Rock
        };

        let player_movement = match player_data_management_type {
            PlayerDataManagementType::AsMovement => match player_movement_data {
                "X" => Movement::Rock,
                "Y" => Movement::Paper,
                "Z" => Movement::Scissor,
                _ => Movement::Rock
            },

            PlayerDataManagementType::AsGuide => match player_movement_data {
                "X" => match enemy_movement { //This means lose
                    Movement::Rock => Movement::Scissor,
                    Movement::Paper => Movement::Rock,
                    Movement::Scissor => Movement::Paper,
                },
                "Y" => match enemy_movement { //This means draw
                    Movement::Rock => Movement::Rock,
                    Movement::Paper => Movement::Paper,
                    Movement::Scissor => Movement::Scissor,
                },
                "Z" => match enemy_movement { //This means win
                    Movement::Rock => Movement::Paper,
                    Movement::Paper => Movement::Scissor,
                    Movement::Scissor => Movement::Rock,
                },
                _ => Movement::Rock
            }
        };


        let strategy = Strategy {
            player_movement,
            enemy_movement,
        };

        return strategy;
    }

    fn calculate_battle_score(&self) -> i32 {
        let comparison_score = match self.player_movement.partial_cmp(&self.enemy_movement) {
            Some(Ordering::Less) => 0,
            Some(Ordering::Equal) => 3,
            Some(Ordering::Greater) => 6,
            None => -1
        };

        return comparison_score;
    }

    fn calculate_base_score(&self) -> i32 {
        let player_base_score = match self.player_movement {
            Movement::Rock => { 1 }
            Movement::Paper => { 2 }
            Movement::Scissor => { 3 }
        };

        return player_base_score;
    }

    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;

        score += self.calculate_base_score();
        score += self.calculate_battle_score();

        return score;
    }
}

pub fn resolve() {
    let path_file = "input/puzzle2.txt";

    let file_content = fs::read_to_string(path_file)
        .expect("Something went wrong reading the file");

    //Part 1
    let mut score_sum = 0;

    for line in file_content.lines() {
        let mut movements_vector: Vec<&str> = vec!();

        for movement in line.split_whitespace() {
            movements_vector.push(movement);
        }

        let strategy = Strategy::new(movements_vector[0], movements_vector[1],
                                     PlayerDataManagementType::AsMovement);

        score_sum += strategy.calculate_score();
    }

    println!("Puzzle 2 Part 1: {}", score_sum);

    //Part 2
    score_sum = 0;

    for line in file_content.lines() {
        let mut movements_vector: Vec<&str> = vec!();

        for movement in line.split_whitespace() {
            movements_vector.push(movement);
        }

        let strategy = Strategy::new(movements_vector[0], movements_vector[1],
                                     PlayerDataManagementType::AsGuide);

        score_sum += strategy.calculate_score();
    }

    println!("Puzzle 2 Part 2: {}", score_sum);
}