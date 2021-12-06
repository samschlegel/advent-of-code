use std::{collections::HashSet, io};

const INPUT: &str = include_str!("../../../inputs/day3.txt");

const LEN: usize = 12;

fn count_bits<'a, S: AsRef<str> + ToString + 'a>(
    lines: impl Iterator<Item = S>,
) -> (HashSet<String>, [i32; LEN], [i32; LEN]) {
    let mut numbers = HashSet::new();
    let mut zeroes = [0; LEN];
    let mut ones = [0; LEN];
    for line in lines {
        for (i, c) in line.as_ref().char_indices() {
            match c {
                '0' => zeroes[i] += 1,
                '1' => ones[i] += 1,
                _ => {}
            }
        }

        numbers.insert(line.to_string());
    }
    (numbers, zeroes, ones)
}

fn part_1(input: &str) {
    let (numbers, mut zeroes, mut ones) = count_bits(input.lines());

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..LEN {
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

    let mut oxygen_candidates = numbers.clone();
    let mut co2_candidates = numbers.clone();
    let mut i = 0;

    while oxygen_candidates.len() > 1 {
        let t = count_bits(oxygen_candidates.iter());
        zeroes = t.1;
        ones = t.2;

        oxygen_candidates.retain(|c| {
            let most_common = if ones[i] >= zeroes[i] { '1' } else { '0' };
            c.chars().nth(i).unwrap() == most_common
        });
        i += 1;
    }

    let oxygen = i32::from_str_radix(oxygen_candidates.iter().next().unwrap(), 2).unwrap();

    println!("Oxygen: {}", oxygen);

    let mut i = 0;

    while co2_candidates.len() > 1 {
        let (_, zeroes, ones) = count_bits(co2_candidates.iter());

        co2_candidates.retain(|c| {
            let least_common = if ones[i] < zeroes[i] { '1' } else { '0' };
            c.chars().nth(i).unwrap() == least_common
        });
        i += 1;
    }

    let co2 = i32::from_str_radix(co2_candidates.iter().next().unwrap(), 2).unwrap();

    println!("co2: {}", co2);

    println!("o2 * co2: {}", oxygen * co2);
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
