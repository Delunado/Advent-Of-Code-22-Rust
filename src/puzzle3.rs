use std::fs;
use std::iter::{successors};

struct Item {
    item_char: char,
    priority: i32,
    appearances: i32,
}

struct Rucksack {
    left_items: Vec<Item>,
    right_items: Vec<Item>,
}

impl Rucksack {
    pub fn new() -> Rucksack {
        let lowercase_iterator = successors(Some('a'), |char| {
            let next = (*char as u8) + 1;
            if next <= b'z' {
                Some(next as char)
            } else {
                None
            }
        });

        let uppercase_iterator = successors(Some('A'), |char| {
            let next = (*char as u8) + 1;
            if next <= b'Z' {
                Some(next as char)
            } else {
                None
            }
        });

        //Fill the rucksack
        let mut items: Vec<Item> = vec![];

        let mut current_priority = 1;

        for item_char in lowercase_iterator.chain(uppercase_iterator) {
            let item_char = item_char;

            items.push(Item {
                item_char: item_char,
                priority: current_priority,
                appearances: 0,
            });

            current_priority += 1;
        }

        let rucksack = Rucksack {
            items
        };

        return rucksack;
    }

    pub fn update_item_appearance(&mut self, item_char: char) -> Option<&Item> {
        for item in &mut self.items {
            if item_char == item.item_char {
                item.appearances += 1;

                return Some(item);
            }
        }

        return None;
    }
}

pub fn resolve() {
    let path_file = "input/puzzle3_example.txt";

    let file_content = fs::read_to_string(path_file)
        .expect("Something went wrong reading the file");

    let mut priority_sum = 0;

    for line in file_content.lines() {
        let mut rucksack = Rucksack::new();

        for char in line.chars() {
            match rucksack.update_item_appearance(char) {
                Some(updated_item) => {
                    if updated_item.appearances == 2 {
                        println!("Updated Item {}, Appearances {}, Priority {}", updated_item.item_char, updated_item.appearances, updated_item.priority);
                        priority_sum += updated_item.priority;
                    }
                }
                None => println!("There is no item with this character in the rucksack!")
            }
        }
    }

    println!("Puzzle 3 Part 1: {}", priority_sum);

    //println!("Puzzle 3 Part 2: {}", score_sum);
}