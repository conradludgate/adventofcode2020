mod parse;

use crate::Challenge;

pub struct Day07 {
    rules: Vec<Rule>,
}

impl Challenge for Day07 {
    fn name() -> &'static str {
        "day07"
    }
    fn new(input: String) -> Self {
        Day07 {
            rules: parse::rules(&input).unwrap().1,
        }
    }
    fn part_one(&self) -> usize {
        let contained_map = rules_into_contained_map(&self.rules);
        can_hold(&contained_map, "shiny gold".to_string()).len()
    }
    fn part_two(&self) -> usize {
        MustContain::new(&self.rules).must_contain("shiny gold".to_string()) - 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    bag_name: String,
    contains: Vec<(usize, String)>,
}

use std::collections::HashMap;
fn rules_into_contain_map(rules: &Vec<Rule>) -> HashMap<String, Vec<(usize, String)>> {
    let mut map = HashMap::new();

    for rule in rules {
        map.insert(rule.bag_name.clone(), rule.contains.clone());
    }

    map
}

fn rules_into_contained_map(rules: &Vec<Rule>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules {
        for (_, contain) in &rule.contains {
            if let Some(v) = map.get_mut(contain) {
                v.push(rule.bag_name.clone());
            } else {
                map.insert(contain.clone(), vec![rule.bag_name.clone()]);
            }
        }
    }

    map
}

use std::collections::HashSet;
fn can_hold(
    contained_map: &HashMap<String, Vec<String>>,
    bag_name: String,
) -> HashSet<String> {
    let mut can_hold_set = HashSet::new();

    if !contained_map.contains_key(&bag_name) {
        return can_hold_set;
    }

    for contained in contained_map[&bag_name].clone() {
        can_hold_set.insert(contained.clone());

        let c = can_hold(contained_map, contained);
        for s in c {
            can_hold_set.insert(s);
        }
    }

    can_hold_set
}

use std::cell::RefCell;
struct MustContain {
    cache: RefCell<HashMap<String, usize>>,
    contain_map: HashMap<String, Vec<(usize, String)>>,
}

impl MustContain {
    pub fn new(rules: &Vec<Rule>) -> Self {
        MustContain {
            cache: RefCell::new(HashMap::new()),
            contain_map: rules_into_contain_map(rules),
        }
    }

    pub fn must_contain(&self, bag_name: String) -> usize {
        {
            if let Some(&count) = self.cache.borrow().get(&bag_name) {
                return count;
            }
        }
        let result = self.contain_map[&bag_name]
            .iter()
            .fold(1, |acc, (amount, name)| {
                acc + amount * self.must_contain(name.clone())
            });

        self.cache.borrow_mut().insert(bag_name, result);

        result
    }
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

    let (_, rules) = parse::rules(input).unwrap();
    let map = rules_into_contain_map(&rules);
    assert_eq!(
        map["light red"],
        vec![(1, "bright white".to_string()), (2, "muted yellow".to_string())]
    );
    assert_eq!(
        map["dark orange"],
        vec![(3, "bright white".to_string()), (4, "muted yellow".to_string())]
    );
    assert_eq!(map["bright white"], vec![(1, "shiny gold".to_string())]);
    assert_eq!(
        map["muted yellow"],
        vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())]
    );
    assert_eq!(
        map["shiny gold"],
        vec![(1, "dark olive".to_string()), (2, "vibrant plum".to_string())]
    );
    assert_eq!(
        map["dark olive"],
        vec![(3, "faded blue".to_string()), (4, "dotted black".to_string())]
    );
    assert_eq!(
        map["vibrant plum"],
        vec![(5, "faded blue".to_string()), (6, "dotted black".to_string())]
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

    let (_, rules) = parse::rules(input).unwrap();
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

    let (_, rules) = parse::rules(input).unwrap();
    let contained_map = rules_into_contained_map(&rules);
    let can_hold_set = can_hold(&contained_map, "shiny gold".to_string());
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

    let (_, rules) = parse::rules(input).unwrap();
    let count = MustContain::new(&rules).must_contain("shiny gold".to_string()) - 1;
    assert_eq!(count, 32);
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

    let (_, rules) = parse::rules(input).unwrap();
    let count = MustContain::new(&rules).must_contain("shiny gold".to_string()) - 1;
    assert_eq!(count, 126);
}
