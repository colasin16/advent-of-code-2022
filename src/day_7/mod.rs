use std::{borrow::Borrow, collections::BTreeMap, io::Error, ops::AddAssign};

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

        let mut sum_dir_sizes = i32::from(0);
        directory_map
            .clone()
            .into_iter()
            .filter(|(_path, size)| size.le(&i32::from(100000)))
            .fold(&mut sum_dir_sizes, |acc, value| {
                acc.add_assign(value.1);
                acc
            });

        print_part(
            1,
            format!(
                "Sum of all directories with size lesser or equal to 100.000: {}",
                sum_dir_sizes
            ),
        );

        const FILE_SYSTEM_MAX_STORAGE: i32 = 70000000;
        const UPDATE_SIZE: i32 = 30000000;

        if let Some(main_dir) = directory_map.get("/") {
            let current_free_space = FILE_SYSTEM_MAX_STORAGE - main_dir;
            let space_needed_to_free_up = UPDATE_SIZE - current_free_space;

            if space_needed_to_free_up.gt(i32::from(0).borrow()) {
                let mut directories_that_can_be_deleted: Vec<(String, i32)> = vec![];
                directory_map
                    .iter()
                    .filter(|entry| entry.1.ge(&space_needed_to_free_up))
                    .fold(&mut directories_that_can_be_deleted, |acc, value| {
                        acc.push((value.0.clone(), value.1.clone()));
                        acc
                    });

                directories_that_can_be_deleted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                if let Some(dir_to_be_deleted) = directories_that_can_be_deleted.first() {
                    print_part(
                        2,
                        format!(
                            "Size of the smallest directory to delete: {}",
                            dir_to_be_deleted.1.to_string()
                        ),
                    );
                }
            }
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
                update_directories_size(directory_map, current_path, &size);
            }
        }
    }
}

fn update_directories_size(
    directory_map: &mut BTreeMap<String, i32>,
    current_path: &String,
    size_to_add: &i32,
) {
    let paths_to_update = current_path.split('/').collect::<Vec<&str>>();
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
