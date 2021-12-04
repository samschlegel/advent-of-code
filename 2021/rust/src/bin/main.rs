use std::{
    env,
    io::{self, BufRead},
};

use aoc_2021::{day4::Day4, Day};

const INPUT: &str = include_str!("../../../inputs/day4.txt");

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut day = match args[1].as_str() {
        "4" => Day4::default(),
        _ => todo!(),
    };

    day.part1(&mut INPUT.lines())?;

    Ok(())
}
