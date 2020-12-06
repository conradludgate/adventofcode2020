use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Copy, Clone)]
struct Policy {
    min: usize,
    max: usize,
    c: char,
}

#[derive(Debug, Clone)]
struct Record<'a> {
    policy: Policy,
    password: &'a str,
}

fn parse_policy(input: &str) -> IResult<&str, Policy> {
    let (input, ((min, max), c)) = separated_pair(
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<usize>()),
            char('-'),
            map_res(digit1, |s: &str| s.parse::<usize>()),
        ),
        char(' '),
        anychar,
    )(input)?;

    Ok((input, Policy { min, max, c }))
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    let (input, (policy, password)) = separated_pair(parse_policy, tag(": "), alpha1)(input)?;

    Ok((input, Record { policy, password }))
}

fn parse_records(input: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(line_ending, parse_record)(input)
}

fn read_file() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");
    input
}

impl<'a> Record<'a> {
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

fn main() {
    let input = read_file();
    let (_, records) = parse_records(&input).unwrap();
    let count = records
        .clone()
        .into_iter()
        .filter(Record::is_valid_1)
        .count();
    println!("valid passwords: {}", count);

    let count = records.into_iter().filter(Record::is_valid_2).count();
    println!("valid passwords: {}", count);
}
