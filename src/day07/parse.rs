use super::Rule;

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

pub fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

pub fn contain(input: &str) -> IResult<&str, (usize, String)> {
    let (input, (n, s)) = separated_pair(
        number,
        space1,
        terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag")))),
    )(input)?;
    Ok((input, (n, s.to_string())))
}

pub fn contains(input: &str) -> IResult<&str, Vec<(usize, String)>> {
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

    Ok((input, Rule { bag_name: bag_name.to_string(), contains }))
}

pub fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(line_ending, rule)(input)
}

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
                bag_name: "light red".to_string(),
                contains: vec![(1, "bright white".to_string()), (2, "muted yellow".to_string())]
            },
            Rule {
                bag_name: "dark orange".to_string(),
                contains: vec![(3, "bright white".to_string()), (4, "muted yellow".to_string())]
            },
            Rule {
                bag_name: "bright white".to_string(),
                contains: vec![(1, "shiny gold".to_string())]
            },
            Rule {
                bag_name: "muted yellow".to_string(),
                contains: vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())]
            },
            Rule {
                bag_name: "shiny gold".to_string(),
                contains: vec![(1, "dark olive".to_string()), (2, "vibrant plum".to_string())]
            },
            Rule {
                bag_name: "dark olive".to_string(),
                contains: vec![(3, "faded blue".to_string()), (4, "dotted black".to_string())]
            },
            Rule {
                bag_name: "vibrant plum".to_string(),
                contains: vec![(5, "faded blue".to_string()), (6, "dotted black".to_string())]
            },
            Rule {
                bag_name: "faded blue".to_string(),
                contains: vec![]
            },
            Rule {
                bag_name: "dotted black".to_string(),
                contains: vec![]
            },
        ]
    )
}
