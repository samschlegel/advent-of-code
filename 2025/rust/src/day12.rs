use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::{is_a, tag};
use nom::character::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::{Finish, IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub width: usize,
    pub height: usize,
    pub quantities: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub shapes: Vec<Shape>,
    pub regions: Vec<Region>,
}

pub fn parse_shape(input: &str) -> IResult<&str, Shape> {
    let (input, _) = digit1().parse_complete(input)?;
    let (input, _) = tag(":\n").parse_complete(input)?;
    let (input, rows) = separated_list1(tag("\n"), is_a("#.")).parse_complete(input)?;

    let height = rows.len();
    let width = rows.first().map_or(0, |r| r.len());
    let cells: Vec<bool> = rows
        .iter()
        .flat_map(|row| row.chars().map(|c| c == '#'))
        .collect();
    let count = cells.iter().filter(|&&cell| cell).count();

    Ok((
        input,
        Shape {
            width,
            height,
            cells,
            count,
        },
    ))
}

pub fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, (width, _, height, _)) = (
        map_res(digit1(), str::parse),
        tag("x"),
        map_res(digit1(), str::parse),
        tag(": "),
    )
        .parse_complete(input)?;
    let (input, quantities) =
        separated_list1(tag(" "), map_res(digit1(), str::parse)).parse_complete(input)?;

    Ok((
        input,
        Region {
            width,
            height,
            quantities,
        },
    ))
}

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, shapes) = separated_list1(tag("\n\n"), parse_shape).parse_complete(input)?;
    let (input, _) = tag("\n\n").parse_complete(input)?;
    let (input, regions) = separated_list1(tag("\n"), parse_region).parse_complete(input)?;
    Ok((input, Input { shapes, regions }))
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, input) = parse_input(input).finish().unwrap();
    assert_eq!(remaining, "", "not all input consumed");
    input
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> usize {
    println!("{:?}", input);
    // Do a basic check if there are enough spaces in the region to fit the shapes if we could just split them all up

    let mut valid = 0;
    for region in &input.regions {
        let total_required = region
            .quantities
            .iter()
            .zip(input.shapes.iter())
            .map(|(quantity, shape)| quantity * shape.count)
            .sum::<usize>();
        if total_required <= region.width * region.height {
            valid += 1;
        }
    }
    println!("Total regions: {}", input.regions.len());
    println!("Valid regions: {}", valid);
    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_solve_part1() {
        let input = input_generator(INPUT);
        let result = solve_part1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parse_shape() {
        let input = "0:
###
##.
##.";
        let (remaining, shape) = parse_shape(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(shape.width, 3);
        assert_eq!(shape.height, 3);
        assert_eq!(
            shape.cells,
            vec![true, true, true, true, true, false, true, true, false]
        );
    }

    #[test]
    fn test_parse_region() {
        let input = "4x4: 0 0 0 0 2 0";
        let (remaining, region) = parse_region(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(region.width, 4);
        assert_eq!(region.height, 4);
        assert_eq!(region.quantities, vec![0, 0, 0, 0, 2, 0]);
    }
}
