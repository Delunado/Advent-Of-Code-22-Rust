use std::fs;

struct Elf {
    calories: i32,
}

impl Elf {
    fn new() -> Elf {
        Elf { calories: 0 }
    }

    pub fn add_calories(&mut self, calories: i32) {
        self.calories += calories;
    }

    pub fn get_calories(&self) -> i32 {
        self.calories
    }
}

pub fn resolve() {
    let path_file = "input/puzzle1.txt";

    let file_content = fs::read_to_string(path_file)
        .expect("Something went wrong reading the file ");


    let mut elves = Vec::new();
    let elf = Elf::new();
    elves.push(elf);

    let mut index = 0;

    for line in file_content.lines() {
        if str::is_empty(line) {
            let elf = Elf::new();
            elves.push(elf);
            index += 1;
        } else {
            let calories = line.parse::<i32>().unwrap();
            elves[index].add_calories(calories);
        }
    }

    //Finding the elf with highest calories.
    let max_calories = elves.iter().map(Elf::get_calories).max().unwrap();

    //Finding the three with more calories
    elves.sort_by(|a, b| b.get_calories().cmp(&a.get_calories()));

    println!("{}", max_calories);
    println!("{}", elves[0].get_calories() + elves[1].get_calories() + elves[2].get_calories());
}