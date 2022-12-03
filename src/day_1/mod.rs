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

        for line in lines {
            if let Ok(snack_calories) = line {
                if snack_calories.parse::<i32>().is_ok() {
                    current_elf_snacks.push(snack_calories.parse().unwrap());
                } else {
                    let total_calories = current_elf_snacks.iter().fold(0, accumulator_sum);
                    elf_calories.push(total_calories);
                    current_elf_snacks.clear();
                }
            } 
        }

        elf_calories.sort();

        if let Some(top_elf_calories) = elf_calories.last() {
            
            println!("The Elf carrying the most calories has {} calories", top_elf_calories);

            let mut top_three_calories: Vec<i32> = Vec::new();
            for i in [1,2,3] {
                if let Some(calories) = elf_calories.get(elf_calories.len() - i) {
                    top_three_calories.push(i32::clone(calories));
                }
            }
            
            let total_top_three_calories = top_three_calories.iter().fold(0, accumulator_sum);
        
            println!("The top three Elves are carrying a total of {} calories", total_top_three_calories);
        }        
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufRead::lines(BufReader::new(file)))
}

fn accumulator_sum(acc: i32, current: &i32) -> i32 {
    acc + current
}