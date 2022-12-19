use std::{borrow::BorrowMut, collections::BTreeMap, io::Error, ops::AddAssign};

use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(7);

    if let Ok(lines) = read_file_lines(input) {
        let terminal_lines: Vec<Result<String, Error>> = lines.collect();
        let lines_text = terminal_lines.iter().map(|predicate| {
            if let Ok(p) = predicate {
                p.to_string()
            } else {
                String::new()
            }
        });

        let mut current_path = String::new();
        let mut directory_map: BTreeMap<String, i32> = BTreeMap::new();
        directory_map.insert(String::from("/"), 0);

        for line in lines_text.clone() {
            if line.starts_with("$") {
                if line.starts_with("$ cd") {
                    change_current_path(&mut current_path, &line);
                }
            } else {
                create_items(&mut directory_map, &mut current_path, line);
            }
        }

        for (dir_path, size) in directory_map.iter() {
            println!("Dir {} has size {}", dir_path, size);
        }
    }
}

fn create_items(
    directory_map: &mut BTreeMap<String, i32>,
    current_path: &mut String,
    item: String,
) {
    let item_parts: Vec<&str> = item.split(" ").collect();

    if item.starts_with("dir") {
        if let Some(dir_name) = item_parts.get(1) {
            let mut path = current_path.clone();

            if !path.eq("/") {
                path.add_assign("/");
            }
            path.add_assign(dir_name);

            directory_map.insert(path, 0);
        }
    } else {
        if let Some(file_size) = item_parts.get(0) {
            let parsed_size = file_size.parse::<i32>();

            if let Ok(size) = parsed_size {
                if let Some(dir_size) = directory_map.get(current_path) {
                    let size_to_add = dir_size + size;
                    directory_map.insert(current_path.clone(), size_to_add);

                    // update_directories_size()
                }
            }
        }
    }
}

fn update_directories_size(
    directory_map: &mut BTreeMap<String, i32>,
    current_directory: &String,
    size_to_add: &i32,
) {
    // Update all parent directories sizes
    if let Some(directory) = directory_map.get(current_directory) {
        if size_to_add > directory {
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

    directory_map
        .into_iter()
        .filter(|entry| paths.clone().into_iter().any(|p| entry.0.eq(&p)).to_owned())
        .for_each(|entry| {
            entry.1.add_assign(size_to_add);
        });
}

fn change_current_path(current_path: &mut String, command_line: &String) {
    let command_args = command_line.split("cd ").collect::<Vec<&str>>();
    match command_args[1] {
        ".." => {
            let mut path_parts: Vec<&str> = current_path.split_inclusive("/").collect();

            path_parts.pop();
            current_path.clone_from(&mut path_parts.join(""));

            if current_path.len() > 1 {
                current_path.pop();
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
}
