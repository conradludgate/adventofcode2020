use parse::Step;

mod parse;
use crate::{Challenge, parsers::{all, lines}};

pub struct Day12 {
    steps: Vec<Step>,
}

impl Challenge for Day12 {
    fn name() -> &'static str {
        "day12"
    }
    fn new(input: String) -> Self {
        Day12 {
            steps: all(lines(parse::step)(&input)),
        }
    }
    fn part_one(&self) -> usize {
        let start = State {
            dir: Dir::East,
            x: 0,
            y: 0,
        };
        let end = self.steps.iter().fold(start, apply_step);
        (end.x.abs() + end.y.abs()) as usize
    }
    fn part_two(&self) -> usize {
        let start = State2 {
            waypoint: Point(10, 1),
            ship: Point(0, 0),
        };
        let end = self.steps.iter().fold(start, apply_step2);
        (end.ship.0.abs() + end.ship.1.abs()) as usize
    }
}


#[repr(u8)]
#[derive(Debug)]
enum Dir {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

fn apply_rotation(dir: Dir, n: isize) -> Dir {
    use Dir::*;
    match (4 + (dir as isize) + n / 90) % 4 {
        0 => East,
        1 => North,
        2 => West,
        _ => South,
    }
}

#[derive(Debug)]
struct State {
    dir: Dir,
    x: isize,
    y: isize,
}

fn apply_step(state: State, step: &Step) -> State {
    let State { dir, x, y } = state;
    use Step::*;
    match step {
        North(n) => State { dir, x, y: y + n },
        East(n) => State { dir, x: x + n, y },
        South(n) => State { dir, x, y: y - n },
        West(n) => State { dir, x: x - n, y },
        Forward(n) => match dir {
            Dir::North => State { dir, x, y: y + n },
            Dir::East => State { dir, x: x + n, y },
            Dir::South => State { dir, x, y: y - n },
            Dir::West => State { dir, x: x - n, y },
        },
        Left(n) => State {
            dir: apply_rotation(dir, *n),
            x,
            y,
        },
        Right(n) => State {
            dir: apply_rotation(dir, -n),
            x,
            y,
        },
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(isize, isize);
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Mul<isize> for Point {
    type Output = Self;
    fn mul(self, other: isize) -> Self::Output {
        Point(self.0 * other, self.1 * other)
    }
}

impl Point {
    fn rotate(self, n: isize) -> Self {
        let Point(x, y) = self;
        match (4 + n / 90) % 4 {
            0 => Point(x, y),
            1 => Point(-y, x),
            2 => Point(-x, -y),
            _ => Point(y, -x),
        }
    }
}

#[derive(Debug)]
struct State2 {
    waypoint: Point,
    ship: Point,
}

fn apply_step2(state: State2, step: &Step) -> State2 {
    let State2 { waypoint, ship } = state;
    use Step::*;
    match *step {
        North(n) => State2 {
            waypoint: waypoint + Point(0, n),
            ship,
        },
        East(n) => State2 {
            waypoint: waypoint + Point(n, 0),
            ship,
        },
        South(n) => State2 {
            waypoint: waypoint + Point(0, -n),
            ship,
        },
        West(n) => State2 {
            waypoint: waypoint + Point(-n, 0),
            ship,
        },
        Forward(n) => State2 {
            waypoint,
            ship: ship + waypoint * n,
        },
        Left(n) => State2 {
            waypoint: waypoint.rotate(n),
            ship,
        },
        Right(n) => State2 {
            waypoint: waypoint.rotate(-n),
            ship,
        },
    }
}
