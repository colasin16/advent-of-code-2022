use std::{char, collections::VecDeque};

use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(5);

    let mut cargo_stacks: [VecDeque<char>; 9] = [
        VecDeque::from(['S', 'P', 'W', 'N', 'J', 'Z']),
        VecDeque::from(['T', 'S', 'G']),
        VecDeque::from(['H', 'L', 'R', 'Q', 'V']),
        VecDeque::from(['D', 'T', 'S', 'V']),
        VecDeque::from(['J', 'M', 'B', 'D', 'T', 'Z', 'Q']),
        VecDeque::from(['L', 'Z', 'C', 'D', 'J', 'T', 'W', 'M']),
        VecDeque::from(['J', 'T', 'G', 'W', 'M', 'P', 'L']),
        VecDeque::from(['H', 'Q', 'F', 'B', 'T', 'M', 'G', 'N']),
        VecDeque::from(['W', 'Q', 'B', 'P', 'C', 'G', 'D', 'R']),
    ];

    if let Ok(lines) = read_file_lines(input) {
        let mut cargo_stacks_copy = cargo_stacks.clone();
        for line in lines {
            if let Ok(instructions_text) = line {
                let instructions: Vec<&str> = instructions_text.split(" ").collect();

                let (amount, from, to) = (
                    instructions[1].parse::<usize>(),
                    instructions[3].parse::<usize>(),
                    instructions[5].parse::<usize>(),
                );

                if amount.is_ok() && from.is_ok() && to.is_ok() {
                    for _m in 0..amount.unwrap() {
                        if let Some(first) =
                            cargo_stacks_copy[from.to_owned().unwrap() - 1].pop_front()
                        {
                            cargo_stacks_copy[to.to_owned().unwrap() - 1].push_front(first);
                        }
                    }
                }
            }
        }

        print_part(
            1,
            format!(
                "The first crate for all stacks are: {}{}{}{}{}{}{}{}{}",
                cargo_stacks_copy[0].front().unwrap(),
                cargo_stacks_copy[1].front().unwrap(),
                cargo_stacks_copy[2].front().unwrap(),
                cargo_stacks_copy[3].front().unwrap(),
                cargo_stacks_copy[4].front().unwrap(),
                cargo_stacks_copy[5].front().unwrap(),
                cargo_stacks_copy[6].front().unwrap(),
                cargo_stacks_copy[7].front().unwrap(),
                cargo_stacks_copy[8].front().unwrap(),
            ),
        );
    }

    if let Ok(lines) = read_file_lines(input) {
        for line in lines {
            if let Ok(instructions_text) = line {
                let instructions: Vec<&str> = instructions_text.split(" ").collect();

                let (amount, from, to) = (
                    instructions[1].parse::<usize>(),
                    instructions[3].parse::<usize>(),
                    instructions[5].parse::<usize>(),
                );

                if amount.is_ok() && from.is_ok() && to.is_ok() {
                    let crates_to_move = cargo_stacks[from.to_owned().unwrap() - 1]
                        .drain(0..amount.to_owned().unwrap())
                        .collect::<VecDeque<char>>();

                    crates_to_move
                        .iter()
                        .rev()
                        .for_each(|c| cargo_stacks[to.to_owned().unwrap() - 1].push_front(*c));
                }
            }
        }

        print_part(
            2,
            format!(
                "The first crate for all stacks using CrateMover9001 are: {}{}{}{}{}{}{}{}{}",
                cargo_stacks[0].front().unwrap(),
                cargo_stacks[1].front().unwrap(),
                cargo_stacks[2].front().unwrap(),
                cargo_stacks[3].front().unwrap(),
                cargo_stacks[4].front().unwrap(),
                cargo_stacks[5].front().unwrap(),
                cargo_stacks[6].front().unwrap(),
                cargo_stacks[7].front().unwrap(),
                cargo_stacks[8].front().unwrap(),
            ),
        );
    }
}
