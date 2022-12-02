use std::fs::File;
use std::io::Result;
use std::io::Lines;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

pub fn execute() {
    let file_path: &Path = Path::new("src/day_1/input.txt");

    if let Ok(lines) = read_lines(file_path) {

        let mut elf_calories: Vec<i16> = Vec::new();
        let mut current_elf_meals: Vec<i16> = Vec::new();
        println!("About to start loop");

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match line {
                Ok(l) => l,
                Err(e) => e,
            }
            // if let Ok(calories) = line {
            //     current_elf_meals.push(calories.parse().unwrap());
            //     println!("{}", calories);
            // } else {
            //     let arr_func = |acc, x| acc + x;
            //     let total_calories = current_elf_meals.iter().fold(0, arr_func);
            //     elf_calories.push(total_calories);
            //     current_elf_meals = Vec::new();
            // }
        }
        println!("Hello, world!");

        elf_calories.sort();
        println!("Most calories are {}", elf_calories[0]);
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufRead::lines(BufReader::new(file)))
}