use crate::util;
use regex::Regex;

pub fn part1() -> i64 {
    let mut count = 0;
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    for line in util::lines_from_path("data/d2.txt") {
        if let Ok(content) = line {
            let captures = re.captures(&content).unwrap();
            let min = captures[1].parse::<usize>().unwrap();
            let max = captures[2].parse::<usize>().unwrap();
            let req_char = &captures[3];
            let pass = &captures[4];

            let char_count = pass.chars().filter(|x| x.to_string() == req_char).count();

            if char_count >= min && char_count <= max {
                count += 1;
            }
        }
    }
    count
}

pub fn part2() -> i64 {
    let mut count = 0;
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    for line in util::lines_from_path("data/d2.txt") {
        if let Ok(content) = line {
            let captures = re.captures(&content).unwrap();
            let first = captures[1].parse::<usize>().unwrap() - 1;
            let second = captures[2].parse::<usize>().unwrap() - 1;
            let req_char = &captures[3];
            let pass = &captures[4];

            let chars: Vec<char> = pass.chars().collect();

            let found_first = chars[first].to_string() == req_char;
            let found_second = chars[second].to_string() == req_char;

            if (found_first && !found_second) || (!found_first && found_second) {
                count += 1;
            }
        }
    }
    count
}
