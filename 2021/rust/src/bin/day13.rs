use std::io;

// const INPUT: &str = include_str!("../../../inputs/day11_test.txt");
const INPUT: &str = include_str!("../../../inputs/day11.txt");

fn parse(s: &str) {
    let lines = s.lines();
    for line in lines {
        if line == "\n" {
            break;
        }
        println!("line");
    }
    for line in lines {}
    loop {
        let line = lines.next();
    }
    loop {
        let line = lines.next();
        if line == "\n" {
            break;
        }
    }
    for line in s.lines() {}
}

fn main() -> io::Result<()> {
    Ok(())
}
