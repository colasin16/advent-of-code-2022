pub fn print_day(day_number: i16) {
    if day_number < 25 {
        println!("├─ Day {}", day_number);
    } else {
        println!("└─ Day {}", day_number);
    }
}

pub fn print_part(part_number: i16, result: String) {
    let part_symbol = if part_number > 1 { "└─" } else { "├─" };
    println!("│   {} Part {}: {}", part_symbol, part_number, result,);
}
