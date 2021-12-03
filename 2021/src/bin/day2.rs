use std::io;

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    aim: i64,
    horizontal: i64,
    depth: i64,
}

fn part_1<I: Iterator<Item = S>, S: AsRef<str>>(lines: I) -> impl Iterator<Item = Position> {
    lines.scan(Position::default(), |state, line| {
        let (command, x_str) = line.as_ref().split_once(' ').unwrap();
        let x: i64 = x_str.parse().unwrap();
        match command {
            "forward" => state.horizontal += x,
            "down" => state.depth += x,
            "up" => state.depth -= x,
            _ => {}
        }
        Some(*state)
    })
}

fn part_2<I: Iterator<Item = S>, S: AsRef<str>>(lines: I) -> impl Iterator<Item = Position> {
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
        Some(*state)
    })
}

const TEST_INPUT: &str = include_str!("../../inputs/day2_test.txt");

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() -> io::Result<()> {
    let final_pos = part_1(INPUT.lines()).last().unwrap();
    println!("{:?}", final_pos);
    println!("Multiplied: {}", final_pos.horizontal * final_pos.depth);

    let final_pos = part_2(INPUT.lines()).last().unwrap();
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
    fn test_part_1() {
        let final_pos = part_1(TEST_INPUT.lines()).last().unwrap();
        assert_eq!(final_pos.horizontal * final_pos.depth, 150);
    }

    #[test]
    fn test_part_2() {
        let final_pos = part_2(TEST_INPUT.lines()).last().unwrap();
        assert_eq!(final_pos.horizontal * final_pos.depth, 900);
    }
}
