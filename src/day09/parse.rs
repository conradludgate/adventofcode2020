use nom::{
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

pub fn numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(line_ending, number)(input)
}
