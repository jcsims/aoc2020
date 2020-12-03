use crate::util;

pub fn part1() -> i64 {
    // We don't do anything with the first line, so offset this
    let mut position = 3;
    let mut trees = 0;

    let mut lines = util::lines_from_path("data/d3.txt");

    // Get width here, plus skip the first row, since we're not starting at a tree.
    let width = lines.next().unwrap().unwrap().len();

    for line in lines {
        if line.unwrap().as_bytes()[(position % width)] as char == '#' {
            trees += 1
        }
        position += 3;
    }

    trees
}

pub fn part2() -> i64 {
    // We don't do anything with the first line, so offset this
    let offsets = vec![1, 3, 5, 7, 1];
    let mut positions = vec![1, 3, 5, 7, 1];
    let mut trees = vec![0, 0, 0, 0, 0];

    let mut lines = util::lines_from_path("data/d3.txt").zip(0..);

    // Get width here, plus skip the first row, since we're not starting at a tree.
    let width = lines.next().unwrap().0.unwrap().len();

    for (line, line_num) in lines {
        let line = line.unwrap();
        let line_bytes = line.as_bytes();
        for i in 0..5 {
            if i != 4 || (i == 4 && line_num % 2 == 0) {
                if line_bytes[(positions[i] % width)] as char == '#' {
                    trees[i] += 1
                }
                positions[i] += offsets[i];
            }
        }
    }

    trees.iter().product()
}
