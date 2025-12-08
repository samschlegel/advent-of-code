use std::{env, fs, io};

use aoc_2021::{day6::Day6, Day};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let day_arg = &args[1];
    let mut day = match day_arg.as_str() {
        // "4" => Box::new(Day4::default()),
        // "5_test" => Day5::new(10, 10),
        // "5" => Day5::new(1000, 1000),
        "6_test" => Day6::new(),
        "6" => Day6::new(),
        _ => todo!(),
    };

    let filepath = format!("inputs/day{}.txt", day_arg);
    let input = fs::read_to_string(filepath)?;

    println!("Part 1:");
    day.part1(&mut input.lines())?;

    println!("Part 2:");
    day.part2(&mut input.lines())?;

    Ok(())
}
