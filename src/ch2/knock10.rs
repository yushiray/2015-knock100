use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::io;

pub fn count_lines(path: &Path) -> usize {
    let file = File::open(path).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines().count()
}

pub fn tab_to_space(path: &Path) {
    let file = File::open(path).expect("file not found");
    let buf = BufReader::new(file);
    for line in buf.lines() {
        let line = line.unwrap();
        println!("{}", &line.replace("\t", " "));
    }
}

pub fn tab_to_space2(path: &Path, tab_width: usize) -> io::Result<String> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let spaces = " ".repeat(tab_width);
    Ok(br.lines().map(|s| match s { Ok(s) => s.replace("\t", &spaces) + "\n", Err(_) => "\0".to_string() }).collect())
}