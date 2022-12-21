use std::{cmp::Ordering, collections::HashSet, io::Error};

use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

pub fn execute(input: &str) {
    print_day(8);

    if let Ok(lines) = read_file_lines(input) {
        let terminal_lines: Vec<Result<String, Error>> = lines.collect();
        let lines_text = terminal_lines.iter().map(|predicate| {
            if let Ok(p) = predicate {
                p.to_string()
            } else {
                String::new()
            }
        });

        let mut trees_row_grid: Vec<Vec<i8>> = vec![];
        let mut trees_column_grid: Vec<Vec<i8>> = vec![];

        for (index, line) in lines_text.clone().enumerate() {
            trees_row_grid.push(vec![]);

            for (i, tree) in line.chars().enumerate() {
                if let Some(tree_height) = tree.to_digit(10) {
                    let tree_to_push = i8::try_from(tree_height).unwrap();

                    trees_row_grid[index].push(tree_to_push);

                    if trees_column_grid.get(i).is_some() {
                        trees_column_grid[i].push(tree_to_push);
                    } else {
                        trees_column_grid.push(vec![tree_to_push]);
                    }
                }
            }
        }

        let mut set_of_visible_trees: HashSet<(usize, usize)> = HashSet::new();

        trees_row_grid
            .clone()
            .iter()
            .enumerate()
            .for_each(|(row_index, tree_row)| {
                set_of_visible_trees.insert((0, row_index));
                set_of_visible_trees.insert((tree_row.len() - 1, row_index));

                let position_of_visible_trees =
                    position_tallest_trees_between_edges(tree_row, SideToIterate::BOTH);

                position_of_visible_trees.iter().for_each(|position| {
                    set_of_visible_trees.insert((*position, row_index));
                })
            });

        trees_column_grid
            .clone()
            .iter()
            .enumerate()
            .for_each(|(col_index, tree_col)| {
                set_of_visible_trees.insert((col_index, 0));
                set_of_visible_trees.insert((col_index, tree_col.len() - 1));

                let position_of_visible_trees =
                    position_tallest_trees_between_edges(tree_col, SideToIterate::BOTH);

                position_of_visible_trees.iter().for_each(|position| {
                    set_of_visible_trees.insert((col_index, *position));
                })
            });

        print_part(
            1,
            format!(
                "Amount of visible trees is {}",
                set_of_visible_trees.into_iter().count()
            ),
        );
    }
}

enum SideToIterate {
    BOTH,
    RIGHT,
    LEFT,
}

fn position_tallest_trees_between_edges(
    list_of_trees: &Vec<i8>,
    side_to_iterate: SideToIterate,
) -> HashSet<usize> {
    let mut position_of_visible_trees: HashSet<usize> = HashSet::new();

    let mut first_found_pos: Option<usize> = list_of_trees
        .clone()
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            if a.cmp(b) == Ordering::Equal {
                return Ordering::Greater;
            } else {
                return a.cmp(b);
            }
        })
        .map(|(index, _)| index);

    let mut last_found_pos: Option<usize> = list_of_trees
        .clone()
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, a), (_, b)| {
            if a.cmp(b) == Ordering::Equal {
                return Ordering::Greater;
            } else {
                return a.cmp(b);
            }
        })
        .map(|(index, _)| index);

    match side_to_iterate {
        SideToIterate::LEFT => iterate_left_side(
            &mut position_of_visible_trees,
            list_of_trees,
            &mut first_found_pos,
        ),
        SideToIterate::RIGHT => iterate_right_side(
            &mut position_of_visible_trees,
            &list_of_trees,
            &mut last_found_pos,
        ),
        SideToIterate::BOTH => {
            if first_found_pos.is_some() && last_found_pos.is_some() {
                if first_found_pos.unwrap().eq(&last_found_pos.unwrap()) {
                    position_of_visible_trees.insert(first_found_pos.unwrap());
                } else {
                    position_of_visible_trees.insert(first_found_pos.unwrap());
                    position_of_visible_trees.insert(last_found_pos.unwrap());
                }

                iterate_left_side(
                    &mut position_of_visible_trees,
                    list_of_trees,
                    &mut first_found_pos,
                );
                iterate_right_side(
                    &mut position_of_visible_trees,
                    list_of_trees,
                    &mut last_found_pos,
                );
            }
        }
    };

    position_of_visible_trees
}

fn iterate_left_side(
    position_of_visible_trees: &mut HashSet<usize>,
    list_of_trees: &Vec<i8>,
    first_found_pos: &mut Option<usize>,
) {
    if let Some(first_pos) = first_found_pos {
        position_of_visible_trees.insert(first_pos.clone());

        let left_final_pos = position_of_visible_trees.iter().min();

        if !left_final_pos.unwrap().eq(&usize::try_from(0).unwrap()) {
            let (left_side, _) = list_of_trees.split_at(*left_final_pos.unwrap());

            if left_side.len() > 1 {
                position_of_visible_trees.extend(position_tallest_trees_between_edges(
                    &left_side.to_vec(),
                    SideToIterate::LEFT,
                ))
            }
        }
    }
}

fn iterate_right_side(
    position_of_visible_trees: &mut HashSet<usize>,
    list_of_trees: &Vec<i8>,
    last_found_pos: &mut Option<usize>,
) {
    if let Some(last_pos) = last_found_pos {
        position_of_visible_trees.insert(last_pos.clone());

        let right_first_pos = position_of_visible_trees.iter().max();

        if !right_first_pos.unwrap().eq(&(list_of_trees.len() - 1)) {
            let (dispensable_side, right_side) =
                list_of_trees.split_at(*right_first_pos.unwrap() + 1);

            if right_side.len() > 1 {
                let right_positions = position_tallest_trees_between_edges(
                    &right_side.to_vec(),
                    SideToIterate::RIGHT,
                );

                let offset_positions = right_positions
                    .iter()
                    .map(|pos| pos + dispensable_side.len());

                position_of_visible_trees.extend(offset_positions);
            }
        }
    }
}
