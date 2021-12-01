use std::fs::File;
use std::io::{self, BufRead};

fn count_increases_part1(input: &[i64]) -> usize {
    input.windows(2).map(|w| (w[1] > w[0]) as usize).sum()
}

fn count_increases_part2(input: &[i64]) -> usize {
    input
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i64>>()
        .windows(2)
        .map(|w| (w[1] > w[0]) as usize)
        .sum()
}

fn main() -> io::Result<()> {
    let file = File::open("../input")?;
    let numbers: Vec<i64> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    println!("part 1: {}", count_increases_part1(&numbers));
    println!("part 2: {}", count_increases_part2(&numbers));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_increases_part1() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increases_part1(&input), 7);
    }

    #[test]
    fn test_count_increases_part2() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increases_part2(&input), 5);
    }
}
