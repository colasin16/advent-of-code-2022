use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};
use std::ops::AddAssign;

mod figure;
use self::figure::{from, Figure, HandShape};

pub fn execute(input: &str) {
    print_day(2);
    const WIN_ROUND_POINTS: i8 = 6;
    const DRAW_ROUND_POINTS: i8 = 3;
    {
        let file_lines = read_file_lines(input);

        if let Ok(lines) = file_lines {
            let mut total_points: i32 = 0;

            for line in lines {
                if let Ok(round) = line {
                    let result: String = round.split(" ").collect();
                    let (elf_figure_symbol, my_figure_symbol) = result.split_at(1);
                    let elf_figure = figure::create_figure(elf_figure_symbol);
                    let my_figure = figure::create_figure(my_figure_symbol);

                    if let (Some(elf_figure), Some(my_figure)) = (elf_figure, my_figure) {
                        total_points.add_assign(i32::from(my_figure.points));

                        if my_figure.is_a_win(&elf_figure) {
                            total_points.add_assign(i32::from(WIN_ROUND_POINTS));
                        } else if my_figure.is_a_draw(&elf_figure) {
                            total_points.add_assign(i32::from(DRAW_ROUND_POINTS));
                        }
                    }
                }
            }

            print_part(1, format!("My total score is {}", total_points));
        }
    }

    let file_lines = read_file_lines(input);

    if let Ok(lines) = file_lines {
        let mut total_points: i32 = 0;

        for line in lines {
            if let Ok(round) = line {
                let result: String = round.split(" ").collect();
                let (elf_figure_symbol, my_figure_symbol) = result.split_at(1);
                let elf_figure = figure::create_figure(elf_figure_symbol);
                let my_action = get_round_action(my_figure_symbol);

                if let (Some(elf_figure), Some(my_action)) = (elf_figure, my_action) {
                    let my_figure = get_figure_by_round_action(&elf_figure, my_action);
                    total_points.add_assign(i32::from(my_figure.points));

                    if my_figure.is_a_win(&elf_figure) {
                        total_points.add_assign(i32::from(WIN_ROUND_POINTS));
                    } else if my_figure.is_a_draw(&elf_figure) {
                        total_points.add_assign(i32::from(DRAW_ROUND_POINTS));
                    }
                }
            }
        }

        print_part(2, format!("My total score now is {}", total_points));
    }
}

enum RoundAction {
    Win,
    Draw,
    Lose,
}

fn get_round_action(action_symbol: &str) -> Option<RoundAction> {
    match action_symbol {
        "Z" => Some(RoundAction::Win),
        "Y" => Some(RoundAction::Draw),
        "X" => Some(RoundAction::Lose),
        _ => None,
    }
}

fn get_figure_by_round_action(opponent_figure: &Figure, action: RoundAction) -> Figure {
    match action {
        RoundAction::Win => match opponent_figure.hand_shape {
            HandShape::Rock => from(HandShape::Paper),
            HandShape::Paper => from(HandShape::Scissors),
            HandShape::Scissors => from(HandShape::Rock),
        },
        RoundAction::Draw => match opponent_figure.hand_shape {
            HandShape::Rock => from(HandShape::Rock),
            HandShape::Paper => from(HandShape::Paper),
            HandShape::Scissors => from(HandShape::Scissors),
        },
        RoundAction::Lose => match opponent_figure.hand_shape {
            HandShape::Rock => from(HandShape::Scissors),
            HandShape::Paper => from(HandShape::Rock),
            HandShape::Scissors => from(HandShape::Paper),
        },
    }
}
