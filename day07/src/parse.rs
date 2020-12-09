use crate::Rule;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    combinator::value,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
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

pub fn contain(input: &str) -> IResult<&str, (usize, &str)> {
    separated_pair(
        number,
        space1,
        terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag")))),
    )(input)
}

pub fn contains(input: &str) -> IResult<&str, Vec<(usize, &str)>> {
    terminated(
        alt((
            value(vec![], tag("no other bags")),
            separated_list1(tag(", "), contain),
        )),
        tag("."),
    )(input)
}

pub fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, bag_name) = terminated(take_until(" bags contain "), tag(" bags contain "))(input)?;
    let (input, contains) = contains(input)?;

    Ok((input, Rule { bag_name, contains }))
}

pub fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(line_ending, rule)(input)
}

mod tests {
    use super::*;
    use crate::Rule;

    #[test]
    fn test_parse_rules() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let (input, rules) = rules(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            rules,
            vec![
                Rule {
                    bag_name: "light red",
                    contains: vec![(1, "bright white"), (2, "muted yellow")]
                },
                Rule {
                    bag_name: "dark orange",
                    contains: vec![(3, "bright white"), (4, "muted yellow")]
                },
                Rule {
                    bag_name: "bright white",
                    contains: vec![(1, "shiny gold")]
                },
                Rule {
                    bag_name: "muted yellow",
                    contains: vec![(2, "shiny gold"), (9, "faded blue")]
                },
                Rule {
                    bag_name: "shiny gold",
                    contains: vec![(1, "dark olive"), (2, "vibrant plum")]
                },
                Rule {
                    bag_name: "dark olive",
                    contains: vec![(3, "faded blue"), (4, "dotted black")]
                },
                Rule {
                    bag_name: "vibrant plum",
                    contains: vec![(5, "faded blue"), (6, "dotted black")]
                },
                Rule {
                    bag_name: "faded blue",
                    contains: vec![]
                },
                Rule {
                    bag_name: "dotted black",
                    contains: vec![]
                },
            ]
        )
    }
}
