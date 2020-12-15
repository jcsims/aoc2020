use crate::util;
use std::collections::{HashMap, HashSet};

pub fn part1() -> i64 {
    let mut bags = HashMap::new();

    for line in util::lines_from_path("data/d7.txt") {
        if let Ok(line) = line {
            let (description, contains) = parse_bag(&line);
            bags.insert(description, contains);
        }
    }

    let mut containers = HashSet::new();
    let mut working_set = HashSet::new();
    let mut next = HashSet::new();

    working_set.insert("shiny gold".to_owned());

    while !working_set.is_empty() {
        for working_bag in working_set {
            for (desc, contained) in &bags {
                if contained.contains_key(&working_bag) {
                    if !containers.contains(desc) {
                        containers.insert(desc.clone());
                        next.insert(desc.clone());
                    }
                }
            }
        }
        working_set = next.clone();
        next.clear();
    }

    containers.len() as i64
}

pub fn part2() -> i64 {
    let mut overall_count = 0;

    let mut bags = HashMap::new();

    for line in util::lines_from_path("data/d7.txt") {
        if let Ok(line) = line {
            let (description, contains) = parse_bag(&line);
            bags.insert(description, contains);
        }
    }

    let mut working_set = HashSet::new();
    working_set.insert((1, "shiny gold".to_owned()));
    let mut next = HashSet::new();

    while !working_set.is_empty() {
        for (multiplier, bag) in working_set {
            if let Some(contained) = bags.get(&bag) {
                for (desc, count) in contained {
                    next.insert((multiplier * count, desc.to_owned()));
                    overall_count += multiplier * count;
                }
            }
        }
        working_set = next.clone();
        next.clear();
    }

    overall_count as i64
}

fn parse_bag(rule: &str) -> (String, HashMap<String, usize>) {
    let words = rule.split_ascii_whitespace();

    let description = words.clone().take(2).collect::<Vec<&str>>().join(" ");

    let mut bag: Vec<&str> = vec![];
    let mut contains = HashMap::new();

    // skip up to the words "bags contain"
    for word in words.skip(4) {
        if word.starts_with("no") {
            break;
        } else if word.starts_with("bag") {
            contains.insert(bag[1..].join(" "), bag[0].parse::<usize>().unwrap());
            bag.clear();
        } else {
            bag.push(word);
        }
    }

    (description, contains)
}

#[test]
fn parse_bag_test() {
    let mut contains = HashMap::new();
    contains.insert("pale blue".to_owned(), 2);
    contains.insert("dark violet".to_owned(), 1);
    assert_eq!(
        ("mirrored gold".to_owned(), contains),
        parse_bag("mirrored gold bags contain 2 pale blue bags, 1 dark violet bag.")
    );
}
