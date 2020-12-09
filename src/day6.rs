use crate::util;
use std::collections::HashSet;

pub fn part1() -> i64 {
    let forms = util::squash_stanzas("data/d6.txt");

    forms.iter().map(|form| one_yes(form)).sum()
}

pub fn part2() -> i64 {
    let forms = util::squash_stanzas("data/d6.txt");

    forms.iter().map(|form| all_yes(form)).sum()
}

fn one_yes(answers: &str) -> i64 {
    let mut set = HashSet::new();
    for c in answers.chars() {
        if c != ' ' {
            set.insert(c);
        }
    }
    set.len() as i64
}

fn all_yes(answers: &str) -> i64 {
    let mut yeses = vec![];

    for answer in answers.split_ascii_whitespace() {
        let mut set = HashSet::new();
        for char in answer.chars() {
            set.insert(char);
        }
        yeses.push(set);
    }

    // picked up the `retain` pointer from https://stackoverflow.com/a/65175232
    let mut mutual = yeses.pop().unwrap();
    mutual.retain(|item| yeses.iter().all(|set| set.contains(item)));

    mutual.len() as i64
}
