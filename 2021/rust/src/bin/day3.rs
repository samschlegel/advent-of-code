use std::io;

const INPUT: &str = include_str!("../../../inputs/day3.txt");

fn part_1(input: &str) {
    let mut zeroes = [0; 12];
    let mut ones = [0; 12];
    for line in input.lines() {
        for (i, c) in line.char_indices() {
            match c {
                '0' => zeroes[i] += 1,
                '1' => ones[i] += 1,
                _ => {}
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..12 {
        gamma <<= 1;
        epsilon <<= 1;

        if zeroes[i] < ones[i] {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("G * E: {}", gamma * epsilon);
}

fn main() -> io::Result<()> {
    part_1(INPUT);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../inputs/day3_test.txt");

    #[test]
    fn test_part_1() {
        part_1(TEST_INPUT);
    }
}
