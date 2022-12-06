use std::ops::AddAssign;

use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(4);

    if let Ok(lines) = read_file_lines(input) {
        let mut amount_of_ranges_fully_overlapped: i32 = 0;
        let mut amount_of_ranges_overlapped: i32 = 0;

        for line in lines {
            if let Ok(assignment_line) = line {
                let assignment_pairs: Vec<&str> = assignment_line.split(",").collect();

                if assignment_pairs.len() == 2 {
                    let range_1 = assignment_pairs[0].split("-").collect::<Vec<&str>>();
                    let range_2 = assignment_pairs[1].split("-").collect::<Vec<&str>>();

                    if range_1.len() == 2 && range_2.len() == 2 {
                        let range_1_start = range_1[0].parse::<i8>().unwrap();
                        let range_1_end = range_1[1].parse::<i8>().unwrap();

                        let range_2_start = range_2[0].parse::<i8>().unwrap();
                        let range_2_end = range_2[1].parse::<i8>().unwrap();

                        let range_1_fully_contains_range_2 =
                            range_1_end >= range_2_end && range_1_start <= range_2_start;

                        let range_2_fully_contains_range_1 =
                            range_2_end >= range_1_end && range_2_start <= range_1_start;

                        if range_1_fully_contains_range_2 || range_2_fully_contains_range_1 {
                            amount_of_ranges_fully_overlapped.add_assign(1);
                        }

                        // Part 2:
                        let mut range_1 = range_1_start..(range_1_end + 1);
                        let range_2 = range_2_start..(range_2_end + 1);

                        if range_1.any(|id| range_2.contains(&id)) {
                            amount_of_ranges_overlapped.add_assign(1);
                        }
                    }
                }
            }
        }

        print_part(1, amount_of_ranges_fully_overlapped.to_string());
        print_part(2, amount_of_ranges_overlapped.to_string());
    }
}
