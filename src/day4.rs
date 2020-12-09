use crate::util;
use regex::Regex;

pub fn part1() -> i64 {
    let passports = util::squash_stanzas("data/d4.txt");

    let expected_fields = vec![
        Regex::new(r"\bbyr:").unwrap(),
        Regex::new(r"\biyr:").unwrap(),
        Regex::new(r"\beyr:").unwrap(),
        Regex::new(r"\bhgt:").unwrap(),
        Regex::new(r"\bhcl:").unwrap(),
        Regex::new(r"\becl:").unwrap(),
        Regex::new(r"\bpid:").unwrap(),
    ];

    let mut correct = 0;
    for passport in passports {
        if expected_fields
            .iter()
            .all(|field| field.is_match(&passport))
        {
            correct += 1;
        }
    }

    correct
}

pub fn part2() -> i64 {
    let passports = util::squash_stanzas("data/d4.txt");

    let expected_fields = vec![
        Regex::new(r"\bbyr:(19[2-9]\d|200[0-2])\b").unwrap(), // four digits; at least 1920
        // and at
        // most 2002.
        Regex::new(r"\biyr:20(1\d|20)\b").unwrap(), // four digits; at least 2010 and at most
        // 2020.
        Regex::new(r"\beyr:20(2\d|30)\b").unwrap(), // four digits; at least 2020 and at most
        // 2030.
        // a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        Regex::new(r"\bhgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\b").unwrap(),
        Regex::new(r"\bhcl:#[a-f0-9]{6}\b").unwrap(), // a # followed by exactly six characters 0-9 or a-f
        Regex::new(r"\becl:amb|blu|brn|gry|grn|hzl|oth\b").unwrap(), // exactly one of: amb blu
        // brn gry grn hzl oth
        Regex::new(r"\bpid:\d{9}\b").unwrap(), // a nine-digit number, including leading zeroes
    ];

    let mut correct = 0;
    for passport in passports {
        if expected_fields
            .iter()
            .all(|field| field.is_match(&passport))
        {
            correct += 1;
        }
    }

    correct
}
