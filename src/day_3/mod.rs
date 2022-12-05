use std::{char, ops::AddAssign};

use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(3);

    if let Ok(lines) = read_file_lines(input) {
        let mut total_points: i32 = 0;

        for line in lines {
            if let Ok(rucksack) = line {
                let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
                let repeated_item = first_compartment
                    .chars()
                    .find(|c| second_compartment.contains(*c));

                if let Some(item) = repeated_item {
                    if let Ok(points) = i32::try_from(get_item_priority(&item)) {
                        total_points.add_assign(points);
                    }
                }
            }
        }

        print_part(1, format!("The sum of the priorities is {}", total_points));
    }
}

fn get_item_priority(item: &char) -> usize {
    let lower_a_to_z_priorities =
        (b'a'..=b'z').fold(String::new(), |acc, c| format!("{}{}", acc, c as char));
    let upper_a_to_z_priorities =
        (b'A'..=b'Z').fold(String::new(), |acc, c| format!("{}{}", acc, c as char));

    let priorities = lower_a_to_z_priorities + &upper_a_to_z_priorities;
    let points = priorities.find(|c| c == item.to_owned());

    match points {
        Some(p) => p + 1,
        None => 0,
    }
}
