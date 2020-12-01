mod day1;
mod util;

use std::env;
use std::time::Instant;

fn main() {
    env_logger::init();
    match env::args().nth(1) {
        None => run_all(),
        Some(exercise) => match exercise.as_ref() {
            "d1p1" => run_one(day1::part1),
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
