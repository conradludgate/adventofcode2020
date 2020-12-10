use nom::{
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn read_file() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");
    input
}

pub fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

pub fn numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(line_ending, number)(input)
}
