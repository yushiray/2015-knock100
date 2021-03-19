
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub fn count_lines(path: &Path) -> usize {
    let file = File::open(path).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines().count()
}
