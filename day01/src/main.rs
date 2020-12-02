use nom::{
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::many1,
    sequence::tuple,
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, usize> {
    let (input, (n, _)) = tuple((map_res(digit1, |s: &str| s.parse::<usize>()), newline))(input)?;

    Ok((input, n))
}

fn parse_input() -> Vec<usize> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");

    let (_, numbers) = many1(parse_number)(&input).expect("could not parse file");
    numbers
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

fn main() {
    let numbers = parse_input();
    let output = find_sum(&numbers, 2020, 2).unwrap();
    assert_eq!(output.len(), 2);
    println!(
        "{} = {}",
        &output
            .iter()
            .fold(String::new(), |a, b| format!("{} x {}", a, b))[3..],
        output.iter().fold(1, |a, b| a * b)
    );

    let output = find_sum(&numbers, 2020, 3).unwrap();
    assert_eq!(output.len(), 3);
    println!(
        "{} = {}",
        &output
            .iter()
            .fold(String::new(), |a, b| format!("{} x {}", a, b))[3..],
        output.iter().fold(1, |a, b| a * b)
    );
}
