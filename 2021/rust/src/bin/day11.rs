use std::io;

use ndarray::{s, Array2, Dim, SliceInfo, SliceInfoElem};

// const INPUT: &str = include_str!("../../../inputs/day11_test.txt");
const INPUT: &str = include_str!("../../../inputs/day11.txt");

type Board = Array2<u8>;

fn parse(s: &str) -> Board {
    let mut width = 0;
    let mut height = 0;
    let mut v = Vec::new();
    for line in s.lines() {
        if width == 0 {
            width = line.len();
        }
        height += 1;

        for cell in line.split("") {
            if cell == "" {
                continue;
            }
            v.push(cell.parse().unwrap());
        }
    }
    Board::from_shape_vec((width, height), v).unwrap()
}

fn increase_all(board: &mut Board) {
    board.mapv_inplace(|e| e + 1);
}

fn neighbors(
    board: &Board,
    x: usize,
    y: usize,
) -> SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 2]>> {
    let (width, height) = board.dim();

    let min_x = (x as i64 - 1).clamp(0, width as i64 - 1) as usize;
    let max_x = (x as i64 + 1).clamp(0, width as i64 - 1) as usize;
    let min_y = (y as i64 - 1).clamp(0, height as i64 - 1) as usize;
    let max_y = (y as i64 + 1).clamp(0, height as i64 - 1) as usize;

    s![min_x..=max_x, min_y..=max_y]
}

fn flash(board: &mut Board) -> usize {
    let mut flashed = 0;
    for x in 0..board.dim().0 {
        for y in 0..board.dim().1 {
            let value = board[(x, y)];
            if value > 9 {
                board.slice_mut(neighbors(board, x, y)).mapv_inplace(|nv| {
                    if nv == 0 {
                        0
                    } else {
                        nv + 1
                    }
                });
                board[(x, y)] = 0;
                flashed += 1;
            }
        }
    }
    flashed
}

fn step(board: &mut Board) -> usize {
    increase_all(board);
    let mut flash_count = 0;
    loop {
        let f = flash(board);
        if f == 0 {
            break;
        };
        flash_count += f;
    }
    flash_count
}

fn main() -> io::Result<()> {
    let mut board = parse(INPUT);
    let mut board2 = board.clone();
    let zeros = Array2::zeros(board.raw_dim());

    println!("{}", board);

    let mut flash_count = 0;
    for _ in 1..=100 {
        flash_count += step(&mut board);
    }
    println!("{}", flash_count);

    let mut steps = 0;
    while board2 != zeros {
        step(&mut board2);
        steps += 1;
    }
    println!("{}", steps);

    Ok(())
}
