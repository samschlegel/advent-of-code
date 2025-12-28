use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::Parser;
use nom::bytes::tag;
use nom::character::digit1;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::separated_list1;

#[derive(Debug, Clone, Copy)]
pub struct Gift {
    pub length: usize,
    pub width: usize,
    pub height: usize,
}

impl Gift {
    pub fn surface_area(&self) -> usize {
        let lw = self.length * self.width;
        let wh = self.width * self.height;
        let hl = self.height * self.length;
        2 * (lw + wh + hl)
    }

    pub fn min_area(&self) -> usize {
        let lw = self.length * self.width;
        let wh = self.width * self.height;
        let hl = self.height * self.length;
        *[lw, wh, hl].iter().min().unwrap()
    }

    pub fn min_perimeter(&self) -> usize {
        let lw = self.length + self.width;
        let wh = self.width + self.height;
        let hl = self.height + self.length;
        2 * *[lw, wh, hl].iter().min().unwrap()
    }

    pub fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Gift> {
    let (input, gifts) = separated_list1(
        tag("\n"),
        map_res(
            (
                digit1::<_, (_, ErrorKind)>(),
                tag("x"),
                digit1::<_, (_, ErrorKind)>(),
                tag("x"),
                digit1::<_, (_, ErrorKind)>(),
            ),
            |(l, _, w, _, h): (&str, &str, &str, &str, &str)| {
                Ok::<Gift, ParseIntError>(Gift {
                    length: l.parse::<usize>()?,
                    width: w.parse::<usize>()?,
                    height: h.parse::<usize>()?,
                })
            },
        ),
    )
    .parse_complete(&input)
    .unwrap();

    assert_eq!(input, "");

    gifts
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Gift>) -> usize {
    input.iter().map(|g| g.surface_area() + g.min_area()).sum()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Gift>) -> usize {
    input.iter().map(|g| g.min_perimeter() + g.volume()).sum()
}
