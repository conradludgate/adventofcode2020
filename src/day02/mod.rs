use crate::Challenge;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub struct Day02 {
    records: Vec<Record>,
}

impl Challenge for Day02 {
    fn name() -> &'static str {
        "day02"
    }
    fn new(input: String) -> Self{
        Day02 {
            records: parse_records(&input).unwrap().1,
        }
    }
    fn part_one(&self) -> usize {
        self.records
            .iter()
            .filter(|&record| record.is_valid_1())
            .count()
    }
    fn part_two(&self) -> usize {
        self.records
            .iter()
            .filter(|&record| record.is_valid_2())
            .count()
    }
}

#[derive(Debug, Copy, Clone)]
struct Policy {
    min: usize,
    max: usize,
    c: char,
}

#[derive(Debug, Clone)]
struct Record {
    policy: Policy,
    password: String,
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_policy(input: &str) -> IResult<&str, Policy> {
    let (input, ((min, max), c)) = separated_pair(
        separated_pair(parse_number, char('-'), parse_number),
        char(' '),
        anychar,
    )(input)?;

    Ok((input, Policy { min, max, c }))
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    let (input, (policy, password)) = separated_pair(parse_policy, tag(": "), alpha1)(input)?;

    Ok((input, Record { policy, password: password.to_string() }))
}

fn parse_records(input: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(line_ending, parse_record)(input)
}

impl Record {
    fn is_valid_1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.policy.c)
            .count();
        self.policy.min <= count && count <= self.policy.max
    }

    fn is_valid_2(&self) -> bool {
        let mut chars = self.password.chars().skip(self.policy.min - 1);
        let a = chars.next().unwrap();
        let b = chars
            .skip(self.policy.max - self.policy.min - 1)
            .next()
            .unwrap();

        (a == self.policy.c) ^ (b == self.policy.c)
    }
}
