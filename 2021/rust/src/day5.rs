use std::{
    io,
    ops::{Add, Sub},
};

use crate::Day;

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<usize>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            cells: vec![0; width * height],
        }
    }

    fn add_line(&mut self, line: &Line) {
        let dir = match line.end - line.start {
            Point { x: 0, y } => (0, y.signum()),
            Point { x, y: 0 } => (x.signum(), 0),
            Point { x, y } if x.abs() == y.abs() => (x.signum(), y.signum()),
            _ => return,
        };

        let mut c = line.start;
        loop {
            self.cells[(c.y * self.width as i32 + c.x) as usize] += 1;

            if c == line.end {
                break;
            }

            c.x += dir.0;
            c.y += dir.1;
        }
    }

    fn count_overlaps(&self) -> usize {
        self.cells.iter().filter(|&&c| c > 1).count()
    }

    // fn print(&self) {
    //     for (i, &c) in self.cells.iter().enumerate() {
    //         if c > 0 {
    //             print!("{}", c);
    //         } else {
    //             print!(".");
    //         }

    //         if (i + 1) % self.width == 0 {
    //             println!();
    //         } else {
    //             print!(" ");
    //         }
    //     }
    // }
}

#[derive(Debug)]
pub struct Day5 {
    board: Board,
}

impl Day5 {
    pub fn new(width: usize, height: usize) -> Self {
        Day5 {
            board: Board::new(width, height),
        }
    }

    fn parse<I, S>(&mut self, _lines: &mut I) -> io::Result<()>
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
    {
        Ok(())
    }
}

impl Day for Day5 {
    fn part1<I, S>(&mut self, lines: &mut I) -> std::io::Result<()>
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
    {
        self.parse(lines)?;

        lines
            .map(|s| {
                let (start_str, end_str) = s.as_ref().split_once(" -> ").unwrap();
                let (start_x, start_y) = start_str.split_once(',').unwrap();
                let (end_x, end_y) = end_str.split_once(',').unwrap();

                Line {
                    start: Point {
                        x: start_x.parse().unwrap(),
                        y: start_y.parse().unwrap(),
                    },
                    end: Point {
                        x: end_x.parse().unwrap(),
                        y: end_y.parse().unwrap(),
                    },
                }
            })
            .for_each(|line| {
                self.board.add_line(&line);
            });

        // self.board.print();
        println!("overlaps: {}", self.board.count_overlaps());

        Ok(())
    }

    fn part2<I, S>(&mut self, lines: &mut I) -> std::io::Result<()>
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
    {
        self.parse(lines)?;

        Ok(())
    }
}
