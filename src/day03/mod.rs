use crate::Challenge;

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    multi::many1,
    multi::separated_list1,
    IResult,
};

pub struct Day03 {
    grid: Grid,
}

impl Challenge for Day03 {
    fn name() -> &'static str {
        "day03"
    }
    fn new(input: String) -> Self{
        Day03 {
            grid: parse_rows(&input).unwrap().1,
        }
    }
    fn part_one(&self) -> usize {
        self.grid.iter((3, 1)).filter(|&spot| spot == Spot::Tree).count()
    }
    fn part_two(&self) -> usize {
        let trees11 = self.grid.iter((1, 1)).filter(|&spot| spot == Spot::Tree).count();
        let trees31 = self.grid.iter((3, 1)).filter(|&spot| spot == Spot::Tree).count();
        let trees51 = self.grid.iter((5, 1)).filter(|&spot| spot == Spot::Tree).count();
        let trees71 = self.grid.iter((7, 1)).filter(|&spot| spot == Spot::Tree).count();
        let trees12 = self.grid.iter((1, 2)).filter(|&spot| spot == Spot::Tree).count();
        trees11 * trees31 * trees51 * trees71 * trees12
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Spot {
    Empty,
    Tree,
}

fn parse_spot(input: &str) -> IResult<&str, Spot> {
    let (input, c) = alt((char('.'), char('#')))(input)?;
    if c == '.' {
        Ok((input, Spot::Empty))
    } else {
        Ok((input, Spot::Tree))
    }
}

#[derive(Debug)]
struct Row(Vec<Spot>);
#[derive(Debug)]
struct Grid(Vec<Row>);

fn parse_row(input: &str) -> IResult<&str, Row> {
    let (input, spots) = many1(parse_spot)(input)?;
    Ok((input, Row(spots)))
}

fn parse_rows(input: &str) -> IResult<&str, Grid> {
    let (input, rows) = separated_list1(line_ending, parse_row)(input)?;
    Ok((input, Grid(rows)))
}

impl Grid {
    fn iter(&self, step: (usize, usize)) -> GridIter {
        GridIter {
            grid: &self,
            pos: (0, 0),
            step,
            width: self.0[0].0.len(),
        }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    pos: (usize, usize),
    step: (usize, usize),
    width: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = Spot;
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.pos;

        if y >= self.grid.0.len() {
            None
        } else {
            let current = self.grid.0[y].0[x];
            let (sx, sy) = self.step;
            self.pos = ((x + sx) % self.width, y + sy);
            Some(current)
        }
    }
}

#[test]
fn test_iter() {
    let (left, grid) = parse_rows(
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
    )
    .unwrap();

    assert_eq!(left.len(), 0);

    let spots: Vec<Spot> = grid.iter((3, 1)).collect();
    use Spot::*;
    assert_eq!(
        spots,
        vec![Empty, Empty, Tree, Empty, Tree, Tree, Empty, Tree, Tree, Tree, Tree]
    );
}
#[test]
fn test_count() {
    let (left, grid) = parse_rows(
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
    )
    .unwrap();

    assert_eq!(left.len(), 0);

    let trees11 = grid.iter((1, 1)).filter(|&spot| spot == Spot::Tree).count();
    let trees31 = grid.iter((3, 1)).filter(|&spot| spot == Spot::Tree).count();
    let trees51 = grid.iter((5, 1)).filter(|&spot| spot == Spot::Tree).count();
    let trees71 = grid.iter((7, 1)).filter(|&spot| spot == Spot::Tree).count();
    let trees12 = grid.iter((1, 2)).filter(|&spot| spot == Spot::Tree).count();

    assert_eq!(trees11, 2);
    assert_eq!(trees31, 7);
    assert_eq!(trees51, 3);
    assert_eq!(trees71, 4);
    assert_eq!(trees12, 2);
}

