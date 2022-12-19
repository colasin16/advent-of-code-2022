use std::{borrow::BorrowMut, collections::BTreeMap, io::Error, ops::AddAssign};

use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(7);

    if let Ok(lines) = read_file_lines(input) {
        let mut current_path = String::new();
        let mut directory_map: BTreeMap<String, i32> = BTreeMap::new();
        let terminal_lines: Vec<Result<String, Error>> = lines.collect();
        let lines_text = terminal_lines.iter().map(|predicate| {
            if let Ok(p) = predicate {
                p.to_string()
            } else {
                String::new()
            }
        });

        let mut indexes_to_skip: Vec<usize> = vec![];

        for (index, line) in lines_text.clone().enumerate() {
            if indexes_to_skip.contains(&index) {
                println!("Skipping line {}", index);
            } else if line.eq("$ ls") {
                println!("Command LS");
                let position_next_command = lines_text
                    .clone()
                    .skip(index + 1)
                    .position(|text| text.starts_with("$"));

                match position_next_command {
                    Some(pos) => {
                        let (range_of_lines, _): (Vec<_>, Vec<_>) = lines_text
                            .clone()
                            .skip(index + 1)
                            .enumerate()
                            .partition(|(i, l)| i < &(&pos));

                        indexes_to_skip.clear();
                        for l in range_of_lines.clone().into_iter() {
                            indexes_to_skip.push(l.0);
                        }

                        let mut ls_items = Vec::new();
                        ls_items = range_of_lines
                            .iter()
                            .map(|(i, l)| String::from(l))
                            .fold(Vec::new().as_mut(), |acc: &mut Vec<String>, value| {
                                acc.push(value);
                                acc
                            })
                            .to_vec();

                        let mut directory_size = 0;

                        create_items(&mut directory_map, &mut directory_size, ls_items.clone());
                        update_directories_size(&mut directory_map, &current_path, &directory_size);
                    }
                    None => {
                        let mut directory_size = 0;
                        create_items(&mut directory_map, &mut directory_size, vec![line]);
                        update_directories_size(&mut directory_map, &current_path, &directory_size);
                    }
                };
            } else if line.starts_with("$ cd") {
                change_current_path(&mut current_path, &line);
            }
        }

        for dir in directory_map.iter() {
            println!("Dir {} size: {}", dir.0, dir.1);
        }
    }
}

fn create_items(
    directory_map: &mut BTreeMap<String, i32>,
    dir_size: &mut i32,
    dir_items: Vec<String>,
) {
    dir_items.iter().for_each(|item| {
        if item.starts_with("dir") {
            directory_map.insert(String::from(item), 0);
            println!("Inserting NEW dir entry - {}", item);
        } else {
            let file: Vec<&str> = item.split(" ").collect();

            if file.len() == 2 {
                let file_size = file[0].parse::<i32>();

                if let Ok(size) = file_size {
                    println!("Adding File to DIR SIZE");
                    dir_size.add_assign(size);
                }
            }
        }
    });
}

fn update_directories_size(
    directory_map: &mut BTreeMap<String, i32>,
    current_directory: &String,
    directory_size: &i32,
) {
    // Update all parent directories sizes
    if let Some(directory) = directory_map.get(current_directory) {
        if directory_size > directory {
            println!("ERROR: A directory that already exists is iterated again with exactly same path but greater size");
        }
    } else {
        println!("ERROR: Directory {} should exist", current_directory);
        directory_map.insert(String::from(current_directory), 0);
    }

    let mut paths_to_update = current_directory.split('/').collect::<Vec<&str>>();
    let mut paths: Vec<String> = vec![];
    paths_to_update
        .clone()
        .into_iter()
        .fold(&mut paths, |acc, value| {
            if acc.len() == 0 {
                acc.push(format!("{}", value));
            } else {
                if let Some(last) = acc.last() {
                    acc.push(format!("{}/{}", last.clone(), value));
                }
            }
            acc
        });

    if paths.len() > 1 {
        paths.splice(0..1, [String::from("/")]);
    }

    println!(
        "Paths to update are: (length is {})",
        paths_to_update.len().to_string()
    );
    for p in &paths {
        println!("{}", p);
    }

    directory_map
        .into_iter()
        .filter(|entry| paths.clone().into_iter().any(|p| entry.0.eq(&p)).to_owned())
        .for_each(|entry| {
            println!("Current dir ({}) SIZE: {}", entry.0, entry.1);
            entry.1.add_assign(directory_size);
            println!("NEW dir ({}) SIZE: {}", entry.0, entry.1);
        });
}

fn change_current_path(current_path: &mut String, command_line: &String) {
    println!("Current Command is: {}", command_line);
    let command_args = command_line.split("cd ").collect::<Vec<&str>>();
    println!("Current path is: {}", current_path);
    match command_args[1] {
        ".." => {
            if let Some(new_path) = current_path.strip_suffix("/") {
                current_path.clone_into(String::from(new_path).borrow_mut());
            }
        }
        "/" => {
            current_path.truncate(0);
            current_path.insert(0, '/');
        }
        _args => {
            if current_path.len() == 1 {
                current_path.insert_str(current_path.len(), format!("{}", _args).as_str());
            } else {
                current_path.insert_str(current_path.len(), format!("/{}", _args).as_str());
            }
        }
    };
    println!("Edited path is: {}", current_path);
}
