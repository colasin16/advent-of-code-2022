use std::io::Error;

use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(7);

    if let Ok(lines) = read_file_lines(input) {
        let mut directory_tree = Directory::new("/", None);
        let terminal_lines: Vec<Result<String, Error>> = lines.collect();
        let lines_text = terminal_lines.iter().map(|predicate| {
            if let Ok(p) = predicate {
                p.to_string()
            } else {
                String::new()
            }
        });

        for (index, line) in lines_text.clone().enumerate() {
            if line.eq("$ ls") {
                let position_next_command = lines_text
                    .clone()
                    .skip(index + 1)
                    .position(|text| text.starts_with("$"));

                if let Some(pos) = position_next_command {
                    let (range_of_lines, _): (Vec<_>, Vec<_>) = lines_text
                        .clone()
                        .skip(index + 1)
                        .enumerate()
                        .partition(|(i, l)| i < &(&pos));

                    let mut ls_items = Vec::new();
                    ls_items = range_of_lines
                        .iter()
                        .map(|(i, l)| String::from(l))
                        .fold(Vec::new().as_mut(), |acc: &mut Vec<String>, value| {
                            acc.push(value);
                            acc
                        })
                        .to_vec();

                    read_command(&mut directory_tree, Command::ls(ls_items.clone()));
                }
            } else if line.starts_with("$ cd") {
                // read_command(&working_directory, Command::cd(line))
            }
        }

        print_dir_children(0, directory_tree);
    }
}

#[derive(Clone)]
struct File {
    name: String,
    size: i32,
}

#[derive(Clone)]
struct Directory {
    parent: Option<Box<Directory>>,
    name: String,
    items: Vec<Item>,
}
impl Directory {
    pub fn new(name: &str, parent: Option<Box<Directory>>) -> Directory {
        Directory {
            parent,
            name: String::from(name),
            items: vec![],
        }
    }
    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

#[derive(Clone)]
enum Item {
    Directory(Directory),
    File(File),
}

enum Command {
    cd(String),
    ls(Vec<String>),
}

fn is_ls_command(text: &String) -> bool {
    text.eq("$ ls")
}

fn is_cd_command(text: &String) -> bool {
    text.starts_with("$ cd")
}

fn read_command(current_directory: &mut Directory, command: Command) {
    match command {
        Command::cd(arg) => {
            if arg == ".." {
                if let Some(parent) = &current_directory.parent {
                    // current_directory = &mut parent.to_owned();
                }
            } else {
                let directory_exists = current_directory.items.iter().any(|item| {
                    if let Item::Directory(dir) = item {
                        dir.name == arg
                    } else {
                        false
                    }
                });
            }
        }
        Command::ls(items) => items.iter().for_each(|item| {
            if item.starts_with("dir") {
                let dir_name = item.get(4..);
                if let Some(name) = dir_name {
                    current_directory.add_item(Item::Directory(Directory::new(
                        name,
                        Some(Box::from(current_directory.clone())),
                    )));
                }
            } else {
                let file: Vec<&str> = item.split(" ").collect();

                if file.len() == 2 {
                    let file_size = file[0].parse::<i32>();
                    let file_name = file[1];

                    if let Ok(size) = file_size {
                        current_directory.add_item(Item::File(File {
                            name: String::from(file_name),
                            size,
                        }));
                    }
                }
            }
        }),
    }
}

fn print_dir_children(level: i16, directory: Directory) {
    println!("{} Directory name: {}", level, directory.name);
    if directory.items.len() > 0 {
        println!("Items: ");
        for item in directory.items {
            match item {
                Item::Directory(dir) => {
                    print_dir_children(level.clone() + 1, dir);
                }
                Item::File(f) => println!("File: {} - {}", f.name, f.size),
            }
        }
    }
}
