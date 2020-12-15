use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::parsers::number;

pub fn bus(input: &str) -> IResult<&str, Option<u128>> {
    alt((value(None, char('x')), map(number, |x| Some(x))))(input)
}

pub fn buses(input: &str) -> IResult<&str, Vec<Option<u128>>> {
    separated_list1(char(','), bus)(input)
}

pub fn input(input: &str) -> IResult<&str, (u128, Vec<Option<u128>>)> {
    separated_pair(number, line_ending, buses)(input)
}
