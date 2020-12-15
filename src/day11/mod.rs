use std::mem::swap;

mod parse;

use crate::{Challenge, parsers::{all, grid}};
pub struct Day11 {
    grid: Vec<Vec<Spot>>,
}

impl Challenge for Day11 {
    fn name() -> &'static str {
        "day11"
    }
    fn new(input: String) -> Self {
        Day11 {
            grid: all(grid(parse::spot)(&input)),
        }
    }
    fn part_one(&self) -> usize {
        run(self.grid.clone())
    }
    fn part_two(&self) -> usize {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Spot {
    Floor,
    Empty,
    Fill,
}

fn neighbours(grid: &Vec<Vec<Spot>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let w = grid[0].len();
    let h = grid.len();

    let xrange = if i == 0 {
        0..=1
    } else if i + 1 == w {
        (i - 1)..=i
    } else {
        (i - 1)..=(i + 1)
    };
    let yrange = if j == 0 {
        0..=1
    } else if j + 1 == h {
        (j - 1)..=j
    } else {
        (j - 1)..=(j + 1)
    };

    for x in xrange.clone() {
        for y in yrange.clone() {
            if x == i && y == j {
                continue;
            }
            count += if grid[y][x] == Spot::Fill { 1 } else { 0 };
        }
    }
    count
}

fn step(grid: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let mut output = grid.clone();
    let w = grid[0].len();
    let h = grid.len();

    for i in 0..w {
        for j in 0..h {
            let neighbours = neighbours(grid, i, j);
            if output[j][i] == Spot::Empty && neighbours == 0 {
                output[j][i] = Spot::Fill;
            } else if output[j][i] == Spot::Fill && neighbours >= 4 {
                output[j][i] = Spot::Empty;
            }
        }
    }

    output
}

fn count(grid: Vec<Vec<Spot>>) -> usize {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|spot| if spot == Spot::Fill { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn run(mut grid: Vec<Vec<Spot>>) -> usize {
    loop {
        let mut output = step(&grid);
        if grid == output {
            return count(grid);
        }
        swap(&mut grid, &mut output);
    }
}

// fn neighbours2(grid: &Vec<Vec<Spot>>, i: usize, j: usize) -> usize {
//     let mut count = 0;
//     let w = grid[0].len();
//     let h = grid.len();

//     let xrange = if i == 0 {
//         0..=1
//     } else if i + 1 == w {
//         (i - 1)..=i
//     } else {
//         (i - 1)..=(i + 1)
//     };
//     let yrange = if j == 0 {
//         0..=1
//     } else if j + 1 == h {
//         (j - 1)..=j
//     } else {
//         (j - 1)..=(j + 1)
//     };

//     for x in xrange.clone() {
//         for y in yrange.clone() {
//             if x == i && y == j {
//                 continue;
//             }
//             count += if grid[y][x] == Spot::Fill { 1 } else { 0 };
//         }
//     }
//     count
// }

#[test]
fn test_neighbours() {
    let input = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";
    let (_, grid) = grid(parse::spot)(input).unwrap();
    assert_eq!(neighbours(&grid, 0, 0), 1);
    assert_eq!(neighbours(&grid, 2, 0), 3);
}

#[test]
fn test_step() {
    let input = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";
    let expected = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";

    let (_, g) = grid(parse::spot)(input).unwrap();
    let (_, expected) = grid(parse::spot)(expected).unwrap();

    assert_eq!(step(&g), expected);
}

#[test]
fn test_run() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let (_, grid) = grid(parse::spot)(input).unwrap();
    let count = run(grid);
    assert_eq!(count, 37);
}
