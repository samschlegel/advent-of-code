use std::{collections::HashSet, fs, io};

// const INPUT: &str = include_str!("../../../inputs/day7_test.txt");
const INPUT: &str = include_str!("../../../inputs/day7.txt");

fn cost(x: i32, y: i32) -> i32 {
    let distance = (x - y).abs();
    (distance * (1 + distance)) / 2
}

fn main() -> io::Result<()> {
    let v: Vec<i32> = INPUT
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut costs = (0..v.len())
        .map(|i| (i, v.iter().map(|&x| cost(x, i as i32)).sum::<i32>()))
        .collect::<Vec<(usize, i32)>>();

    costs.sort_by_key(|t| t.1);

    println!("{:?}", v.iter().sum::<i32>() / v.len() as i32);
    println!("{:?}", &costs[0..5]);
    Ok(())
}
