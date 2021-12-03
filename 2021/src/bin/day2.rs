use std::io;

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    aim: i64,
    horizontal: i64,
    depth: i64,
}

fn part_a<I: Iterator<Item = S>, S: AsRef<str>>(lines: I) -> impl Iterator<Item = Position> {
    lines.scan(Position::default(), |state, line| {
        let (command, x_str) = line.as_ref().split_once(' ').unwrap();
        let x: i64 = x_str.parse().unwrap();
        match command {
            "forward" => state.horizontal += x,
            "down" => state.depth += x,
            "up" => state.depth -= x,
            _ => {}
        }
        Some(state.clone())
    })
}

fn part_b<I: Iterator<Item = S>, S: AsRef<str>>(lines: I) -> impl Iterator<Item = Position> {
    lines.scan(Position::default(), |state, line| {
        let (command, x_str) = line.as_ref().split_once(' ').unwrap();
        let x: i64 = x_str.parse().unwrap();
        match command {
            "forward" => {
                state.horizontal += x;
                state.depth += state.aim * x;
            }
            "down" => state.aim += x,
            "up" => state.aim -= x,
            _ => {}
        }
        Some(state.clone())
    })
}

const TEST_INPUT: &str = include_str!("../../inputs/day2_test.txt");

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() -> io::Result<()> {
    let final_pos = part_a(INPUT.lines()).last().unwrap();
    println!("{:?}", final_pos);
    println!("Multiplied: {}", final_pos.horizontal * final_pos.depth);

    let final_pos = part_b(INPUT.lines()).last().unwrap();
    println!("{:?}", final_pos);
    println!("Multiplied: {}", final_pos.horizontal * final_pos.depth);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part_a() {
        part_a(TEST_INPUT.lines()).for_each(|x| {
            println!("{:?}", x);
        });
    }
}
