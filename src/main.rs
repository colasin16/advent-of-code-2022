mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod helpers;

fn main() {
    println!("Advent of Code");
    day_1::execute("src/day_1/input.txt");
    day_2::execute("src/day_2/input.txt");
    day_3::execute("src/day_3/input.txt");
    day_4::execute("src/day_4/input.txt");
    day_5::execute("src/day_5/input.txt");
    day_6::execute("src/day_6/input.txt");
}
