use std::io;

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    horizontal: f32,
    depth: f32,
}

fn part_a<I: Iterator<Item = S>, S: AsRef<str>>(lines: I) -> impl Iterator<Item = Position> {
    lines.scan(Position::default(), |state, line| {
        let (command, x_str) = line.as_ref().split_once(' ').unwrap();
        let x: f32 = x_str.parse().unwrap();
        match command {
            "forward" => state.horizontal += x,
            "down" => state.depth += x,
            "up" => state.depth -= x,
            _ => {}
        }
        Some(state.clone())
    })
}

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() -> io::Result<()> {
    let final_pos = part_a(INPUT.lines()).last().unwrap();
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
