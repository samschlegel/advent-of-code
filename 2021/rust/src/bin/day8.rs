use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use maplit::hashset;

const INPUT: &str = include_str!("../../../inputs/day8_test.txt");
// const INPUT: &str = include_str!("../../../inputs/day8.txt");
lazy_static! {
    static ref NUMBERS: HashSet<&'static str> = hashset![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    static ref CHARS: Vec<char> = "abcdefg".chars().collect_vec();
}

fn check_permutation(permutation: &[&u8], mangled_nums: &[&str]) -> bool {
    mangled_nums.iter().all(|mangled_num| {
        let unmangled = String::from_utf8(
            mangled_num
                .as_bytes()
                .iter()
                .map(|&c| *permutation[(c - 97) as usize])
                .collect::<Vec<u8>>(),
        )
        .unwrap();
        if mangled_num.len() > 2 {
            println!("{:?} {} {}", permutation, mangled_num, unmangled);
        }
        NUMBERS.contains(unmangled.as_str())
    })
}

fn main() -> io::Result<()> {
    INPUT.lines().for_each(|line| {
        let split = line.split_whitespace().collect_vec();

        let permutation = "abcdefg"
            .as_bytes()
            .into_iter()
            .permutations(7)
            .find(|permutation| check_permutation(permutation, &split[0..10]))
            .unwrap();

        println!("{:?}", permutation);
    });

    Ok(())
}
