use crate::util;

pub fn part1() -> i64 {
    util::lines_from_path("data/d5.txt")
        .map(|line| {
            let (row, seat) = decode_seat(&line.unwrap());
            row * 8 + seat
        })
        .max()
        .unwrap()
}

pub fn part2() -> i64 {
    let mut lines = util::lines_from_path("data/d5.txt")
        .map(|line| {
            let (row, seat) = decode_seat(&line.unwrap());
            row * 8 + seat
        })
        .collect::<Vec<i64>>();

    lines.sort();

    let mut seat = 0;

    lines.iter().fold(0, |last, &next| {
        if next - last == 2 {
            seat = last + 1;
        }
        next
    });

    seat
}

fn shrink_space(encoding: &str, lower: i64, upper: i64) -> i64 {
    let mut diff;
    let mut l = lower;
    let mut u = upper;
    for c in encoding.chars() {
        diff = (u - l + 1) / 2;
        match c {
            'F' | 'L' => u -= diff,
            'B' | 'R' => l += diff,
            a => panic!("unknown char {}", a),
        }
    }
    l
}

fn decode_seat(encoding: &str) -> (i64, i64) {
    let row = shrink_space(&encoding[0..7], 0, 127);
    let seat = shrink_space(&encoding[7..10], 0, 7);

    (row, seat)
}

#[test]
fn shrink_test() {
    let row = "FBFBBFF";
    assert_eq!(44, shrink_space(row, 0, 127));

    let seat = "RLR";
    assert_eq!(5, shrink_space(seat, 0, 7));
}

#[test]
fn decode_test() {
    let encoding = "FBFBBFFRLR";
    assert_eq!((44, 5), decode_seat(&encoding));
}
