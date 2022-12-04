use std::fs;

enum EnemyPlay {
    A, //Rock
    B, //Paper
    C, //Scissor
}

enum PlayerPlay {
    X, //Rock
    Y, //Paper
    Z, //Scissor
}

struct Strategy {
    enemy_play: EnemyPlay,
    player_play: PlayerPlay,
}

impl Strategy {
    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;
        
        let player_play_score = match self.player_play {
            PlayerPlay::X => {1}
            PlayerPlay::Y => {2}
            PlayerPlay::Z => {3}
        };
        
        score += player_play_score;
        
        let battle_score = match self.player_play {
            PlayerPlay::X => {
                let versus_enemy_score = match self.enemy_play {
                    EnemyPlay::A => {1}
                    EnemyPlay::B => {0}
                    EnemyPlay::C => {2}
                };
                
                versus_enemy_score
            }
            PlayerPlay::Y => {
                let versus_enemy_score = match self.enemy_play {
                    EnemyPlay::A => {2}
                    EnemyPlay::B => {1}
                    EnemyPlay::C => {0}
                };

                versus_enemy_score
            }
            PlayerPlay::Z => {
                let versus_enemy_score = match self.enemy_play {
                    EnemyPlay::A => {0}
                    EnemyPlay::B => {2}
                    EnemyPlay::C => {1}
                };

                versus_enemy_score
            }
        };
        
        score += battle_score;
        
        score
    }
}

pub fn resolve() {
    let path_file = "input/puzzle2.txt";

    let file_content = fs::read_to_string(path_file)
        .expect("Something went wrong reading the file");
    
    
}