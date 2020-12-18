mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

use std::env;
use std::time::Instant;

fn main() {
    match env::args().nth(1) {
        None => run_all(),
        Some(exercise) => match exercise.as_ref() {
            "d1p1" => run_one(day1::part1),
            "d1p2" => run_one(day1::part2),
            "d2p1" => run_one(day2::part1),
            "d2p2" => run_one(day2::part2),
            "d3p1" => run_one(day3::part1),
            "d3p2" => run_one(day3::part2),
            "d4p1" => run_one(day4::part1),
            "d4p2" => run_one(day4::part2),
            "d5p1" => run_one(day5::part1),
            "d5p2" => run_one(day5::part2),
            "d6p1" => run_one(day6::part1),
            "d6p2" => run_one(day6::part2),
            "d7p1" => run_one(day7::part1),
            "d7p2" => run_one(day7::part2),
            "d8p1" => run_one(day8::part1),
            "d8p2" => run_one(day8::part2),
            "d9p1" => run_one(day9::part1),
            "d9p2" => run_one(day9::part2),
            _ => panic!("unknown exercise: {}", exercise),
        },
    }
}

fn run_all() {
    let start = Instant::now();

    assert_eq!(
        691_771,
        run_one_and_return("day1::part1", day1::part1),
        "day1::part1 failed!"
    );

    assert_eq!(
        232_508_760,
        run_one_and_return("day1::part2", day1::part2),
        "day1::part2 failed!"
    );

    assert_eq!(
        418,
        run_one_and_return("day2::part1", day2::part1),
        "day2::part1 failed!"
    );

    assert_eq!(
        616,
        run_one_and_return("day2::part2", day2::part2),
        "day2::part2 failed!"
    );

    assert_eq!(
        216,
        run_one_and_return("day3::part1", day3::part1),
        "day3::part1 failed!"
    );

    assert_eq!(
        6_708_199_680,
        run_one_and_return("day3::part2", day3::part2),
        "day3::part2 failed!"
    );

    assert_eq!(
        216,
        run_one_and_return("day4::part1", day4::part1),
        "day4::part1 failed!"
    );

    assert_eq!(
        150,
        run_one_and_return("day4::part2", day4::part2),
        "day4::part2 failed!"
    );

    assert_eq!(
        953,
        run_one_and_return("day5::part1", day5::part1),
        "day5::part1 failed!"
    );

    assert_eq!(
        615,
        run_one_and_return("day5::part2", day5::part2),
        "day5::part2 failed!"
    );

    assert_eq!(
        6930,
        run_one_and_return("day6::part1", day6::part1),
        "day6::part1 failed!"
    );

    assert_eq!(
        3585,
        run_one_and_return("day6::part2", day6::part2),
        "day6::part2 failed!"
    );

    assert_eq!(
        233,
        run_one_and_return("day7::part1", day7::part1),
        "day7::part1 failed!"
    );

    assert_eq!(
        421550,
        run_one_and_return("day7::part2", day7::part2),
        "day7::part2 failed!"
    );

    assert_eq!(
        1709,
        run_one_and_return("day8::part1", day8::part1),
        "day8::part1 failed!"
    );

    assert_eq!(
        1976,
        run_one_and_return("day8::part2", day8::part2),
        "day8::part2 failed!"
    );

    assert_eq!(
        41_682_220,
        run_one_and_return("day9::part1", day9::part1),
        "day9::part1 failed!"
    );

    assert_eq!(
        5_388_976,
        run_one_and_return("day9::part2", day9::part2),
        "day9::part2 failed!"
    );

    println!("Total elapsed time: {:?}", Instant::elapsed(&start));
}

fn run_one(exercise: fn() -> i64) {
    let now = Instant::now();

    println!("{}", exercise());

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}

fn run_one_and_return(name: &str, exercise: fn() -> i64) -> i64 {
    let now = Instant::now();

    let result = exercise();

    println!("Elapsed time for {}: {:?}", name, Instant::elapsed(&now));

    result
}
