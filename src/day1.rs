use crate::util;
use std::collections::HashSet;

pub fn part1() -> i64 {
    let mut nums = HashSet::new();
    for line in util::lines_from_path("data/d1.txt") {
        if line.is_ok() {
            let current = line.unwrap().parse::<i64>().unwrap();
            let target = 2020 - current;
            if nums.contains(&target) {
                return current * target;
            }
            nums.insert(current);
        }
    }

    42
}

pub fn part2() -> i64 {
    let mut nums = HashSet::new();
    for line in util::lines_from_path("data/d1.txt") {
        if line.is_ok() {
            nums.insert(line.unwrap().parse::<i64>().unwrap());
        }
    }

    for i in &nums {
        for j in &nums {
            let rem = 2020 - i - j;
            if nums.contains(&rem) {
                return rem * i * j;
            }
        }
    }
    42
}
