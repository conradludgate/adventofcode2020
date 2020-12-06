use nom::{
    alt,
    character::complete::{char, line_ending, one_of},
    complete,
    multi::count,
    multi::separated_list1,
    named,
    sequence::separated_pair,
    tag, IResult, InputTakeAtPosition,
};

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
enum Field {
    BirthYear = 0x01,
    IssueYear = 0x02,
    ExpirationYear = 0x04,
    Height = 0x08,
    HairColor = 0x10,
    EyeColor = 0x20,
    PassportID = 0x40,
    CountryID = 0x80,
}

named!(parse_field<&str, Field>, alt!(
    complete!(tag!("byr")) => { |_| Field::BirthYear } |
    complete!(tag!("iyr")) => { |_| Field::IssueYear } |
    complete!(tag!("eyr")) => { |_| Field::ExpirationYear } |
    complete!(tag!("hgt")) => { |_| Field::Height } |
    complete!(tag!("hcl")) => { |_| Field::HairColor } |
    complete!(tag!("ecl")) => { |_| Field::EyeColor } |
    complete!(tag!("pid")) => { |_| Field::PassportID } |
    complete!(tag!("cid")) => { |_| Field::CountryID }
));

#[derive(Debug, PartialEq, Copy, Clone)]
struct FieldData<'a> {
    field: Field,
    data: &'a str,
}

pub fn parse_data(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(|item| item.is_ascii_whitespace())
}

fn parse_field_data(input: &str) -> IResult<&str, FieldData> {
    let (input, (field, data)) = separated_pair(parse_field, char(':'), parse_data)(input)?;
    Ok((input, FieldData { field, data }))
}

#[derive(Debug, PartialEq, Clone)]
struct Passport<'a>(Vec<FieldData<'a>>);

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let (input, fields) = separated_list1(one_of("\n\r\t "), parse_field_data)(input)?;
    Ok((input, Passport(fields)))
}

fn parse_passports(input: &str) -> IResult<&str, Vec<Passport>> {
    separated_list1(count(line_ending, 2), parse_passport)(input)
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

fn parse_colour(input: &str) -> IResult<&str, Vec<char>> {
    let (input, _) = char('#')(input)?;
    let (input, colour) = count(one_of("0123456789abcdef"), 6)(input)?;
    Ok((input, colour))
}

impl<'a> FieldData<'a> {
    fn is_valid(&self) -> bool {
        match self.field {
            Field::BirthYear => {
                if let Ok(year) = self.data.parse::<usize>() {
                    1920 <= year && year <= 2002
                } else {
                    false
                }
            }
            Field::IssueYear => {
                if let Ok(year) = self.data.parse::<usize>() {
                    2010 <= year && year <= 2020
                } else {
                    false
                }
            }
            Field::ExpirationYear => {
                if let Ok(year) = self.data.parse::<usize>() {
                    2020 <= year && year <= 2030
                } else {
                    false
                }
            }
            Field::Height => {
                if self.data.ends_with("cm") {
                    if let Ok(height) = self.data[..self.data.len() - 2].parse::<usize>() {
                        150 <= height && height <= 193
                    } else {
                        false
                    }
                } else if self.data.ends_with("in") {
                    if let Ok(height) = self.data[..self.data.len() - 2].parse::<usize>() {
                        59 <= height && height <= 76
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Field::HairColor => {
                if self.data.len() != 7 {
                    false
                } else {
                    if let Ok((rest, hex)) = parse_colour(self.data) {
                        rest.len() == 0 && hex.len() == 6
                    } else {
                        false
                    }
                }
            }
            Field::EyeColor => match self.data {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            Field::PassportID => {
                if let Ok(_) = self.data.parse::<usize>() {
                    self.data.len() == 9
                } else {
                    false
                }
            }
            Field::CountryID => true,
        }
    }
}

impl<'a> Passport<'a> {
    fn into_bits(&self) -> u8 {
        self.0.iter().fold(0, |a, fd| a | fd.field as u8)
    }
    fn has_correct_fields(&self) -> bool {
        let bits = self.into_bits();
        const MASK: u8 = 0x7f;
        bits & MASK == MASK
    }
    fn is_valid(&self) -> bool {
        self.has_correct_fields() && self.0.iter().fold(true, |a, fd| a && fd.is_valid())
    }
}

fn main() {
    let input = read_file();
    let (_, passports) = parse_passports(&input).expect("could not parse file");
    let valid_fields = passports
        .clone()
        .into_iter()
        .filter(Passport::has_correct_fields)
        .count();
    println!("valid fields: {:?}", valid_fields);

    let valid = passports.into_iter().filter(Passport::is_valid).count();
    println!("valid: {:?}", valid)
}

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
        let (input, field) = parse_field(input).unwrap();
        assert_eq!(input.len(), 0);
        assert_eq!(field, expected);
    }
}

#[test]
fn test_parse_field_data() {
    let inputs = vec!["byr:1971", "hgt:170cm", "hcl:#ff0000"];

    for input in inputs.into_iter() {
        let (input, _) = parse_field_data(input).unwrap();
        assert_eq!(input.len(), 0);
    }
}

#[test]
fn test_parse_passport() {
    let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
    let (input, fields) = parse_passport(input).unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(fields.0.len(), 7);
}

#[test]
fn test_valid() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    let (input, passports) = parse_passports(input).unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(passports.len(), 4);

    let valid_passports: Vec<bool> = passports.iter().map(Passport::has_correct_fields).collect();
    assert_eq!(valid_passports, vec![true, false, true, false]);

    let valid_passports: Vec<u8> = passports.iter().map(Passport::into_bits).collect();
    assert_eq!(valid_passports, vec![0xff, 0xf7, 0x7f, 0x7e]);
}
