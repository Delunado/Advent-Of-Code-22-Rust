use std::fs;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

enum PlayerDataManagementType {
    AsMovement,
    AsGuide,
}

#[derive(PartialEq, Eq, Ord)]
enum Movement {
    //Rock, same as X
    A,

    //Paper, same as Y
    B,

    //Scissor, same as Z
    C,
}

impl PartialOrd for Movement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Movement::A, Movement::A) => Some(Equal),
            (Movement::B, Movement::B) => Some(Equal),
            (Movement::C, Movement::C) => Some(Equal),

            (Movement::A, Movement::B) => Some(Less),
            (Movement::A, Movement::C) => Some(Greater),

            (Movement::B, Movement::A) => Some(Greater),
            (Movement::B, Movement::C) => Some(Less),

            (Movement::C, Movement::A) => Some(Less),
            (Movement::C, Movement::B) => Some(Greater)
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
            "A" => Movement::A,
            "B" => Movement::B,
            "C" => Movement::C,
            _ => Movement::A
        };


        let player_movement;

        player_movement = match player_data_management_type {
            PlayerDataManagementType::AsMovement => match player_movement_data {
                "X" => Movement::A,
                "Y" => Movement::B,
                "Z" => Movement::C,
                _ => Movement::A
            },

            PlayerDataManagementType::AsGuide => match player_movement_data {
                "X" => match enemy_movement { //This means lose
                    Movement::A => Movement::C,
                    Movement::B => Movement::A,
                    Movement::C => Movement::B,
                },
                "Y" => match enemy_movement { //This means draw
                    Movement::A => Movement::A,
                    Movement::B => Movement::B,
                    Movement::C => Movement::C,
                },
                "Z" => match enemy_movement { //This means win
                    Movement::A => Movement::B,
                    Movement::B => Movement::C,
                    Movement::C => Movement::A,
                },
                _ => Movement::A
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
            Movement::A => { 1 }
            Movement::B => { 2 }
            Movement::C => { 3 }
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