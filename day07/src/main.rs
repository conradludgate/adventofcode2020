use cached::proc_macro::cached;
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

#[derive(Debug, Clone, PartialEq)]
struct Rule<'a> {
    bag_name: &'a str,
    contains: Vec<(usize, &'a str)>,
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_contain(input: &str) -> IResult<&str, (usize, &str)> {
    separated_pair(
        parse_number,
        space1,
        terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag")))),
    )(input)
}

fn parse_contains(input: &str) -> IResult<&str, Vec<(usize, &str)>> {
    terminated(
        alt((
            value(vec![], tag("no other bags")),
            separated_list1(tag(", "), parse_contain),
        )),
        tag("."),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, bag_name) = terminated(take_until(" bags contain "), tag(" bags contain "))(input)?;
    let (input, contains) = parse_contains(input)?;

    Ok((input, Rule { bag_name, contains }))
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(line_ending, parse_rule)(input)
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

use std::collections::HashMap;
fn rules_into_contain_map<'a>(rules: &Vec<Rule<'a>>) -> HashMap<&'a str, Vec<(usize, &'a str)>> {
    let mut map = HashMap::new();

    for rule in rules {
        map.insert(rule.bag_name, rule.contains.clone());
    }

    map
}

fn rules_into_contained_map<'a>(rules: &Vec<Rule<'a>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

    for rule in rules {
        for (_, contain) in &rule.contains {
            if let Some(v) = map.get_mut(contain) {
                v.push(rule.bag_name);
            } else {
                map.insert(contain, vec![rule.bag_name]);
            }
        }
    }

    map
}

use std::collections::HashSet;
fn can_hold<'a>(
    contained_map: &HashMap<&'a str, Vec<&'a str>>,
    bag_name: &str,
) -> HashSet<&'a str> {
    let mut can_hold_set = HashSet::new();

    if !contained_map.contains_key(bag_name) {
        return can_hold_set;
    }

    for &contained in &contained_map[bag_name] {
        can_hold_set.insert(contained);

        let c = can_hold(contained_map, contained);
        for s in c {
            can_hold_set.insert(s);
        }
    }

    can_hold_set
}

fn must_contain<'a>(
    contain_map: &HashMap<&'a str, Vec<(usize, &'a str)>>,
    bag_name: &str,
) -> usize {
    let mut count = 1;
    for &(amount, bag_name) in &contain_map[bag_name] {
        count += amount * must_contain(contain_map, bag_name)
    }
    count
}

fn main() {
    let input = read_file();
    let (_, rules) = parse_rules(&input).unwrap();
    let contained_map = rules_into_contained_map(&rules);
    let can_hold_set = can_hold(&contained_map, "shiny gold");
    println!(
        "{:?} different bags can hold 'shiny gold'",
        can_hold_set.len()
    );

    let contain_map = rules_into_contain_map(&rules);
    let count = must_contain(&contain_map, "shiny gold") - 1;
    println!("shiny gold bag must contain {:?} total bags", count);
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

    let (input, rules) = parse_rules(input).unwrap();
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

#[test]
fn test_rules_into_contain_map() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let (_, rules) = parse_rules(input).unwrap();
    let map = rules_into_contain_map(&rules);
    assert_eq!(
        map["light red"],
        vec![(1, "bright white"), (2, "muted yellow")]
    );
    assert_eq!(
        map["dark orange"],
        vec![(3, "bright white"), (4, "muted yellow")]
    );
    assert_eq!(map["bright white"], vec![(1, "shiny gold")]);
    assert_eq!(
        map["muted yellow"],
        vec![(2, "shiny gold"), (9, "faded blue")]
    );
    assert_eq!(
        map["shiny gold"],
        vec![(1, "dark olive"), (2, "vibrant plum")]
    );
    assert_eq!(
        map["dark olive"],
        vec![(3, "faded blue"), (4, "dotted black")]
    );
    assert_eq!(
        map["vibrant plum"],
        vec![(5, "faded blue"), (6, "dotted black")]
    );
    assert_eq!(map["faded blue"], vec![]);
    assert_eq!(map["dotted black"], vec![]);
}

#[test]
fn test_rules_into_contained_map() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let (_, rules) = parse_rules(input).unwrap();
    let map = rules_into_contained_map(&rules);
    assert_eq!(map["bright white"], vec!["light red", "dark orange"]);
    assert_eq!(map["muted yellow"], vec!["light red", "dark orange"]);
    assert_eq!(map["shiny gold"], vec!["bright white", "muted yellow"]);
    assert_eq!(map["dark olive"], vec!["shiny gold"]);
    assert_eq!(map["vibrant plum"], vec!["shiny gold"]);
    assert_eq!(
        map["faded blue"],
        vec!["muted yellow", "dark olive", "vibrant plum"]
    );
    assert_eq!(map["dotted black"], vec!["dark olive", "vibrant plum"]);
}

#[test]
fn test_can_hold() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let (_, rules) = parse_rules(input).unwrap();
    let contained_map = rules_into_contained_map(&rules);
    let can_hold_set = can_hold(&contained_map, "shiny gold");
    assert_eq!(can_hold_set.len(), 4);
    assert!(can_hold_set.contains("light red"));
    assert!(can_hold_set.contains("dark orange"));
    assert!(can_hold_set.contains("bright white"));
    assert!(can_hold_set.contains("muted yellow"));
}

#[test]
fn test_must_contain() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let (_, rules) = parse_rules(input).unwrap();
    let contain_map = rules_into_contain_map(&rules);
    let count = must_contain(&contain_map, "shiny gold");
    assert_eq!(count - 1, 32);
}

#[test]
fn test_must_contain2() {
    let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    let (_, rules) = parse_rules(input).unwrap();
    let contain_map = rules_into_contain_map(&rules);
    let count = must_contain(&contain_map, "shiny gold");
    assert_eq!(count - 1, 126);
}
