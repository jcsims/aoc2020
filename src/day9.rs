use crate::util::lines_from_path;
use std::collections::VecDeque;

pub fn part1() -> i64 {
    let nums = lines_from_path("data/d9.txt")
        .map(|num| num.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut working: VecDeque<i64> = nums[0..25].to_vec().into();

    'outer: for num in nums.iter().skip(25) {
        for &op in &working {
            if num - op == op {
                continue;
            }
            if working.contains(&(num - op)) {
                working.push_back(*num);
                working.pop_front();
                continue 'outer;
            }
        }
        return num.clone();
    }

    panic!("didn't find a bad number");
}

pub fn part2() -> i64 {
    let target = 41_682_220;

    let nums = lines_from_path("data/d9.txt")
        .map(|num| num.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for i in 0..nums.len() {
        for j in (i + 2)..nums.len() {
            let slice = &nums[i..j];
            if slice.iter().sum::<i64>() == target {
                return slice.iter().min().unwrap() + slice.iter().max().unwrap();
            }
        }
    }

    panic!("didn't find a contiguous set!");
}
