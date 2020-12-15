use nom::{branch::alt, character::complete::char, combinator::map, sequence::preceded, IResult};

use crate::parsers::number;

pub enum Step {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

pub fn step(input: &str) -> IResult<&str, Step> {
    use Step::*;
    alt((
        map(preceded(char('N'), number), |x| North(x)),
        map(preceded(char('E'), number), |x| East(x)),
        map(preceded(char('S'), number), |x| South(x)),
        map(preceded(char('W'), number), |x| West(x)),
        map(preceded(char('L'), number), |x| Left(x)),
        map(preceded(char('R'), number), |x| Right(x)),
        map(preceded(char('F'), number), |x| Forward(x)),
    ))(input)
}
