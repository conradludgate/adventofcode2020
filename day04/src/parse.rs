use crate::{Field, FieldData, Passport};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, one_of},
    combinator::{map_res, value},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult, InputTakeAtPosition,
};

pub fn data(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(|item| item.is_ascii_whitespace())
}

pub fn field(input: &str) -> IResult<&str, Field> {
    alt((
        value(Field::BirthYear, tag("byr")),
        value(Field::IssueYear, tag("iyr")),
        value(Field::ExpirationYear, tag("eyr")),
        value(Field::Height, tag("hgt")),
        value(Field::HairColor, tag("hcl")),
        value(Field::EyeColor, tag("ecl")),
        value(Field::PassportID, tag("pid")),
        value(Field::CountryID, tag("cid")),
    ))(input)
}

pub fn field_data(input: &str) -> IResult<&str, FieldData> {
    let (input, (field, data)) = separated_pair(field, char(':'), data)(input)?;
    Ok((input, FieldData { field, data }))
}

pub fn passport(input: &str) -> IResult<&str, Passport> {
    let (input, fields) = separated_list1(one_of("\n\r\t "), field_data)(input)?;
    Ok((input, Passport(fields)))
}

pub fn passports(input: &str) -> IResult<&str, Vec<Passport>> {
    separated_list1(count(line_ending, 2), passport)(input)
}

pub fn read_file() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("input.txt").expect("could not open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("could not read file");
    input
}

pub fn hex_colour(input: &str) -> IResult<&str, Vec<char>> {
    let (input, _) = char('#')(input)?;
    count(one_of("0123456789abcdef"), 6)(input)
}

pub fn eye_colour(input: &str) -> IResult<&str, &str> {
    alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input)
}
#[derive(Debug, Copy, Clone)]
pub enum Height {
    Centimetres(usize),
    Inches(usize),
}

pub fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

pub fn height(input: &str) -> IResult<&str, Height> {
    let (input, number) = number(input)?;
    alt((
        value(Height::Centimetres(number), tag("cm")),
        value(Height::Inches(number), tag("in")),
    ))(input)
}

mod tests {
    use super::*;
    use crate::Field;
    #[test]
    fn test_parse_field() {
        let inputs = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        use Field::*;
        let expected = vec![
            BirthYear,
            IssueYear,
            ExpirationYear,
            Height,
            HairColor,
            EyeColor,
            PassportID,
            CountryID,
        ];

        for (input, expected) in inputs.into_iter().zip(expected.into_iter()) {
            let (input, field) = field(input).unwrap();
            assert_eq!(input.len(), 0);
            assert_eq!(field, expected);
        }
    }

    #[test]
    fn test_parse_field_data() {
        let inputs = vec!["byr:1971", "hgt:170cm", "hcl:#ff0000"];

        for input in inputs.into_iter() {
            let (input, _) = field_data(input).unwrap();
            assert_eq!(input.len(), 0);
        }
    }

    #[test]
    fn test_parse_passport() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        let (input, fields) = passport(input).unwrap();
        assert_eq!(input.len(), 0);
        assert_eq!(fields.0.len(), 7);
    }
}
