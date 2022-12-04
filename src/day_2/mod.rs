use std::ops::AddAssign;

use crate::helpers::file_system::read_file_lines;

pub fn execute(input: &str) {
    println!("Day 2 ------");
    const WIN_ROUND_POINTS: i8 = 6;
    const DRAW_ROUND_POINTS: i8 = 3;

    if let Ok(rounds) = read_file_lines(input) {
        let mut total_points: i32 = 0;

        for round in rounds {
            if let Ok(round_figures) = round {
                let result: String = round_figures.split(" ").collect();
                let (elf_figure_symbol, my_figure_symbol) = result.split_at(1);
                let elf_figure = create_figure(elf_figure_symbol);
                let my_figure = create_figure(my_figure_symbol);
                
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

        println!("My total score is {}", total_points)
    }
}

#[derive(PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors
}

struct Figure {
    hand_shape: HandShape,
    points: i8
}
impl Figure {
    fn is_a_win(&self, opponent_figure: &Figure) -> bool {
        if matches!(self.hand_shape, HandShape::Rock) && matches!(opponent_figure.hand_shape, HandShape::Scissors) {
            true
        } else if matches!(self.hand_shape, HandShape::Paper) && matches!(opponent_figure.hand_shape, HandShape::Rock) {
            true
        } else if matches!(self.hand_shape, HandShape::Scissors) && matches!(opponent_figure.hand_shape, HandShape::Paper) {
            true
        } else {
            false
        }
    }

    fn is_a_draw(&self, opponent_figure: &Figure) -> bool {
        self.hand_shape == opponent_figure.hand_shape
    }
}

fn create_figure(symbol: &str) -> Option<Figure> {
    if ["A", "X"].contains(&symbol) {
        let figure = Figure {
            hand_shape: HandShape::Rock,
            points: 1
        };
        Some(figure)
    } else if ["B", "Y"].contains(&symbol) {
        let figure = Figure {
            hand_shape: HandShape::Paper,
            points: 2
        };
        Some(figure)
    } else if ["C", "Z"].contains(&symbol) {
        let figure = Figure {
            hand_shape: HandShape::Scissors,
            points: 3
        };
        Some(figure)
    } else {
        None
    }
}