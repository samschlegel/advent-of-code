use aoc_2021_day1::*;
use std::io;

fn main() -> io::Result<()> {
    let numbers = load_input("../input");

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
