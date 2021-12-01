use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
