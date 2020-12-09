mod parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule<'a> {
    bag_name: &'a str,
    contains: Vec<(usize, &'a str)>,
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

use std::cell::RefCell;
struct MustContain<'a> {
    cache: RefCell<HashMap<&'a str, usize>>,
    contain_map: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> MustContain<'a> {
    pub fn new(rules: &Vec<Rule<'a>>) -> Self {
        MustContain {
            cache: RefCell::new(HashMap::new()),
            contain_map: rules_into_contain_map(rules),
        }
    }

    pub fn must_contain(&self, bag_name: &'a str) -> usize {
        {
            if let Some(&count) = self.cache.borrow().get(bag_name) {
                return count;
            }
        }
        let result = self.contain_map[bag_name]
            .iter()
            .fold(1, |acc, (amount, name)| {
                acc + amount * self.must_contain(name)
            });

        self.cache.borrow_mut().insert(bag_name, result);

        result
    }
}

fn main() {
    let input = parse::read_file();
    let (_, rules) = parse::rules(&input).unwrap();
    let contained_map = rules_into_contained_map(&rules);
    let can_hold_set = can_hold(&contained_map, "shiny gold");
    println!(
        "{:?} different bags can hold 'shiny gold'",
        can_hold_set.len()
    );

    let count = MustContain::new(&rules).must_contain("shiny gold") - 1;
    println!("shiny gold bag must contain {:?} total bags", count);
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

    let (_, rules) = parse::rules(input).unwrap();
    let count = MustContain::new(&rules).must_contain("shiny gold") - 1;
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
    let count = MustContain::new(&rules).must_contain("shiny gold") - 1;
    assert_eq!(count, 126);
}
