mod day1;
mod day2;
mod util;

use std::env;
use std::time::Instant;

fn main() {
    env_logger::init();
    match env::args().nth(1) {
        None => run_all(),
        Some(exercise) => match exercise.as_ref() {
            "d1p1" => run_one(day1::part1),
            "d1p2" => run_one(day1::part2),
            "d2p1" => run_one(day2::part1),
            "d2p2" => run_one(day2::part2),
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

    println!("Total elapsed time: {:?}", Instant::elapsed(&start));
}
e
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
