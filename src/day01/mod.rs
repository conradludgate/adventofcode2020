use crate::Challenge;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub struct Day01 {
    numbers: Vec<usize>,
}

impl Challenge for Day01 {
    fn name() -> &'static str {
        "day01"
    }
    fn new(input: String) -> Self {
        Day01 {
            numbers: parse_numbers(&input).unwrap().1,
        }
    }
    fn part_one(&self) -> usize {
        find_sum(&self.numbers, 2020, 2).unwrap().iter().product()
    }
    fn part_two(&self) -> usize {
        find_sum(&self.numbers, 2020, 3).unwrap().iter().product()
    }
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(line_ending, parse_number)(input)
}

fn find_sum<T>(numbers: &[T], sum: T, n: usize) -> Option<Vec<T>>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + PartialOrd + Copy,
{
    match n {
        0 => Some(vec![]),
        1 => {
            if numbers.contains(&sum) {
                Some(vec![sum])
            } else {
                None
            }
        }
        _ => {
            for i in 0..numbers.len() {
                if numbers[i] < sum {
                    let attempt = find_sum(&numbers[i + 1..], sum - numbers[i], n - 1);
                    if let Some(mut v) = attempt {
                        v.push(numbers[i]);
                        return Some(v);
                    }
                }
            }
            None
        }
    }
}

#[test]
fn find_sum_pair_test() {
    let output = find_sum(&vec![1721, 979, 366, 299, 675, 1456], 2020, 2);
    assert_eq!(output, Some(vec![299, 1721]));
}

#[test]
fn find_sum_trio_test() {
    let output = find_sum(&vec![1721, 979, 366, 299, 675, 1456], 2020, 3);
    assert_eq!(output, Some(vec![675, 366, 979]));
}
