use std::fs::File;
use std::io::Result;
use std::io::Lines;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

pub fn execute() {
    let file_path: &Path = Path::new("src/day_1/input.txt");

    if let Ok(lines) = read_lines(file_path) {

        let mut elf_calories: Vec<i32> = Vec::new();
        let mut current_elf_snacks: Vec<i32> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(snack_calories) = line {
                if snack_calories.parse::<i32>().is_ok() {
                    current_elf_snacks.push(snack_calories.parse().unwrap());
                } else {
                    let total_calories = current_elf_snacks.iter().fold(0, |acc, current| acc + current);
                    elf_calories.push(total_calories);
                    current_elf_snacks.clear();
                }
            } 
        }

        elf_calories.sort();
        println!("Most calories are {}", elf_calories.last().unwrap());
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufRead::lines(BufReader::new(file)))
}
