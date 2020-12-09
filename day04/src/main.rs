mod parse;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Field {
    BirthYear = 0x01,
    IssueYear = 0x02,
    ExpirationYear = 0x04,
    Height = 0x08,
    HairColor = 0x10,
    EyeColor = 0x20,
    PassportID = 0x40,
    CountryID = 0x80,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldData<'a> {
    field: Field,
    data: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Passport<'a>(Vec<FieldData<'a>>);

impl<'a> FieldData<'a> {
    fn is_valid(&self) -> bool {
        use nom::combinator::{complete, recognize};
        use parse::{eye_colour, height, hex_colour, number, Height::*};
        match self.field {
            Field::BirthYear => {
                complete(number)(self.data).map_or(false, |(_, year)| 1920 <= year && year <= 2002)
            }
            Field::IssueYear => {
                complete(number)(self.data).map_or(false, |(_, year)| 2010 <= year && year <= 2020)
            }
            Field::ExpirationYear => {
                complete(number)(self.data).map_or(false, |(_, year)| 2020 <= year && year <= 2030)
            }
            Field::Height => {
                complete(height)(self.data).map_or(false, |(_, height)| match height {
                    Centimetres(cm) => 150 <= cm && cm <= 193,
                    Inches(cm) => 59 <= cm && cm <= 76,
                })
            }
            Field::HairColor => complete(hex_colour)(self.data).is_ok(),
            Field::EyeColor => complete(eye_colour)(self.data).is_ok(),
            Field::PassportID => {
                recognize(complete(number))(self.data).map_or(false, |(_, input)| input.len() == 9)
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
    let input = parse::read_file();
    let (_, passports) = parse::passports(&input).expect("could not parse file");
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

    let (input, passports) = parse::passports(input).unwrap();
    assert_eq!(input.len(), 0);
    assert_eq!(passports.len(), 4);

    let valid_passports: Vec<bool> = passports.iter().map(Passport::has_correct_fields).collect();
    assert_eq!(valid_passports, vec![true, false, true, false]);

    let valid_passports: Vec<u8> = passports.iter().map(Passport::into_bits).collect();
    assert_eq!(valid_passports, vec![0xff, 0xf7, 0x7f, 0x7e]);
}
