use crate::Day;

#[derive(Debug)]
pub struct Day6 {
    lanternfish: [usize; 9],
}

impl Day6 {
    pub fn new() -> Self {
        Day6 {
            lanternfish: [0; 9],
        }
    }

    fn parse<I, S>(&mut self, _lines: &mut I) -> std::io::Result<()>
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
    {
        _lines
            .next()
            .unwrap()
            .as_ref()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .for_each(|l| self.lanternfish[l] += 1);

        Ok(())
    }

    pub fn step(&mut self) {
        let new_lanternfish = &mut [0; 9];
        for i in 0..9 {
            let count = self.lanternfish[i];
            if i == 0 {
                new_lanternfish[8] += count;
                new_lanternfish[6] += count;
            } else {
                new_lanternfish[i - 1] += count;
            }
        }
        self.lanternfish = *new_lanternfish;
    }
}

impl Day for Day6 {
    fn part1<I, S>(&mut self, lines: &mut I) -> std::io::Result<()>
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
    {
        self.parse(lines)?;

        for _ in 0..80 {
            self.step();
        }

        println!("count: {}", self.lanternfish.iter().sum::<usize>());

        Ok(())
    }

    fn part2<I, S>(&mut self, lines: &mut I) -> std::io::Result<()>
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
    {
        self.parse(lines)?;

        for _ in 0..256 {
            self.step();
        }

        println!("count: {}", self.lanternfish.iter().sum::<usize>());

        Ok(())
    }
}
