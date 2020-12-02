use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1, newline},
    combinator::map_res,
    multi::many1,
    IResult,
};

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

fn read_policy(input: &str) -> IResult<&str, Policy> {
    use std::str::FromStr;

    let (input, min) = map_res(digit1, usize::from_str)(input)?;
    let (input, _) = char('-')(input)?;
    let (input, max) = map_res(digit1, usize::from_str)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, c) = anychar(input)?;

    Ok((input, Policy { min, max, c }))
}

fn read_record(input: &str) -> IResult<&str, Record> {
    let (input, policy) = read_policy(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, password) = alpha1(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        Record {
            policy,
            password: password.to_string(),
        },
    ))
}

fn read_records(input: &str) -> IResult<&str, Vec<Record>> {
    many1(read_record)(input)
}

fn parse_file() -> Vec<Record> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");

    let (_, records) = read_records(&input).expect("could not parse file");

    records
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

fn main() {
    let records = parse_file();
    let count = records
        .clone()
        .into_iter()
        .filter(Record::is_valid_1)
        .count();
    println!("valid passwords: {}", count);

    let count = records.into_iter().filter(Record::is_valid_2).count();
    println!("valid passwords: {}", count);
}
