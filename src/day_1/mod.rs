use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(1);

    if let Ok(lines) = read_file_lines(input) {
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
            print_part(
                1,
                format!("Elf with most calories has {} calories", top_elf_calories),
            );

            let mut top_three_calories: Vec<i32> = Vec::new();
            for i in [1, 2, 3] {
                if let Some(calories) = elf_calories.get(elf_calories.len() - i) {
                    top_three_calories.push(i32::clone(calories));
                }
            }

            let total_top_three_calories = top_three_calories.iter().fold(0, accumulator_sum);

            print_part(
                2,
                format!(
                    "Top three Elves are carrying a total of {} calories",
                    total_top_three_calories
                ),
            );
        }
    }
}

fn accumulator_sum(acc: i32, current: &i32) -> i32 {
    acc + current
}
