#[derive(PartialEq)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}
pub fn from(hand_shape: HandShape) -> Figure {
    match hand_shape {
        HandShape::Rock => Figure {
            hand_shape: HandShape::Rock,
            points: 1,
        },
        HandShape::Paper => Figure {
            hand_shape: HandShape::Paper,
            points: 2,
        },
        HandShape::Scissors => Figure {
            hand_shape: HandShape::Scissors,
            points: 3,
        },
    }
}

pub struct Figure {
    pub hand_shape: HandShape,
    pub points: i8,
}
impl Figure {
    pub fn is_a_win(&self, opponent_figure: &Figure) -> bool {
        if matches!(self.hand_shape, HandShape::Rock)
            && matches!(opponent_figure.hand_shape, HandShape::Scissors)
        {
            true
        } else if matches!(self.hand_shape, HandShape::Paper)
            && matches!(opponent_figure.hand_shape, HandShape::Rock)
        {
            true
        } else if matches!(self.hand_shape, HandShape::Scissors)
            && matches!(opponent_figure.hand_shape, HandShape::Paper)
        {
            true
        } else {
            false
        }
    }

    pub fn is_a_draw(&self, opponent_figure: &Figure) -> bool {
        self.hand_shape == opponent_figure.hand_shape
    }
}

pub fn create_figure(symbol: &str) -> Option<Figure> {
    match symbol {
        "A" | "X" => Some(Figure {
            hand_shape: HandShape::Rock,
            points: 1,
        }),
        "B" | "Y" => Some(Figure {
            hand_shape: HandShape::Paper,
            points: 2,
        }),
        "C" | "Z" => Some(Figure {
            hand_shape: HandShape::Scissors,
            points: 3,
        }),
        _ => None,
    }
}
