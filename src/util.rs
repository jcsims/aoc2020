use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn lines_from_path(filepath: &str) -> io::Lines<std::io::BufReader<std::fs::File>> {
    let file = File::open(Path::new(filepath)).unwrap();

    BufReader::new(file).lines()
}

pub fn squash_stanzas(filepath: &str) -> Vec<String> {
    let lines = lines_from_path(filepath);
    let mut stanzas = vec![];
    let mut current = String::new();

    for line in lines {
        let s = line.unwrap();
        if s.is_empty() {
            stanzas.push(current.clone());
            current.clear();
        } else {
            current.push(' ');
            current.push_str(&s);
        }
    }

    // Make sure we don't lose the last one
    stanzas.push(current);
    stanzas
}
