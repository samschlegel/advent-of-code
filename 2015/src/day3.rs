use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn input_generator(input: &str) -> String {
    input.to_string()
}

struct State {
    x: isize,
    y: isize,
    visited: std::collections::HashMap<(isize, isize), usize>,
}

impl State {
    pub fn new() -> Self {
        State {
            x: 0,
            y: 0,
            visited: std::collections::HashMap::new(),
        }
    }

    pub fn move_by(&mut self, direction: char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => {}
        }
    }

    pub fn visit(&mut self) {
        self.visited
            .entry((self.x, self.y))
            .and_modify(|ct| *ct += 1)
            .or_insert(1);
    }
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    let mut state = State::new();
    state.visit();
    for c in input.chars() {
        state.move_by(c);
        state.visit();
    }
    state.visited.len()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    let mut state1 = State::new();
    let mut state2 = State::new();
    state1.visit();
    state2.visit();
    for c in input.chars() {
        state1.move_by(c);
        state1.visit();

        // swap state1 and state2
        std::mem::swap(&mut state1, &mut state2);
    }
    // merge visited maps, adding values together instead of just replacing
    state1.visited.keys().for_each(|k| {
        state2
            .visited
            .entry(*k)
            .and_modify(|ct| *ct += state1.visited[k])
            .or_insert(state1.visited[k]);
    });
    state2.visited.len()
}
