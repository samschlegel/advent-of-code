use std::io;

use crate::Day;

#[derive(Debug)]
struct Board {
    unmarked_sum: usize,
    lookup: [Option<(usize, usize)>; 100],
    row_counts: [usize; 5],
    col_counts: [usize; 5],
    winning_number: Option<usize>,
}

impl Board {
    fn parse<I: Iterator<Item = S>, S: AsRef<str>>(lines: &mut I) -> io::Result<Self> {
        let mut board = Board {
            unmarked_sum: 0,
            lookup: [None; 100],
            row_counts: [0; 5],
            col_counts: [0; 5],
            winning_number: None,
        };

        for row in 0..5 {
            let line = lines.next().unwrap();
            let mut numbers = line.as_ref().split_whitespace();
            for col in 0..5 {
                let number: usize = numbers.next().unwrap().parse().unwrap();
                board.unmarked_sum += number;
                board.lookup[number] = Some((row, col));
            }
        }

        Ok(board)
    }

    fn mark_number(&mut self, number: usize) -> bool {
        if let Some((row, col)) = self.lookup[number] {
            self.row_counts[row] += 1;
            self.col_counts[col] += 1;
            self.unmarked_sum -= number;

            if self.row_counts[row] == 5 || self.col_counts[col] == 5 {
                self.winning_number = Some(number);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct Day4 {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Day4 {
    fn parse<I: Iterator<Item = S>, S: AsRef<str>>(&mut self, lines: &mut I) -> io::Result<()> {
        self.numbers = lines
            .next()
            .unwrap()
            .as_ref()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        lines.next();
        loop {
            let board = Board::parse(lines)?;
            self.boards.push(board);
            if lines.next().is_none() {
                break;
            }
        }

        Ok(())
    }
}

impl Day for Day4 {
    fn part1<I: Iterator<Item = S>, S: AsRef<str>>(
        &mut self,
        lines: &mut I,
    ) -> std::io::Result<()> {
        self.parse(lines)?;

        for (round, number) in self.numbers.iter().enumerate() {
            let mut winners = Vec::new();
            for (i, board) in self.boards.iter_mut().enumerate() {
                if board.winning_number.is_some() {
                    continue;
                }

                if board.mark_number(*number) {
                    winners.push(i);
                }
            }

            if !winners.is_empty() {
                for idx in winners {
                    let board = &self.boards[idx];
                    println!(
                        "Round {}: Board {} won: {} * {} = {}",
                        round,
                        idx,
                        board.unmarked_sum,
                        number,
                        board.unmarked_sum * number
                    );
                }
            }
        }

        Ok(())
    }

    fn part2<I: Iterator<Item = S>, S: AsRef<str>>(
        &mut self,
        lines: &mut I,
    ) -> std::io::Result<()> {
        self.parse(lines)?;

        for number in &self.numbers {
            let mut winners = Vec::new();
            for (i, board) in self.boards.iter_mut().enumerate() {
                if board.mark_number(*number) {
                    winners.push(i);
                }
            }

            if !winners.is_empty() {
                for idx in winners {
                    let board = &self.boards[idx];
                    println!(
                        "Board {} won: {} * {} = {}",
                        idx,
                        board.unmarked_sum,
                        number,
                        board.unmarked_sum * number
                    );
                }

                break;
            }
        }

        Ok(())
    }
}
