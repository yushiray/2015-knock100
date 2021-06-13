use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::collections::HashSet;
use std::io::ErrorKind;
use std::io;
use std::io::Error;

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

pub fn cut(input_file_name: &Path, num: usize, output_file_name: &str) {
    let input_f = File::open(input_file_name).expect("file not found");
    let read_buf = BufReader::new(input_f);
    let mut output_f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_name)
        .expect(format!("can't open file[{}] with write option", output_file_name).as_str());
    read_buf.lines().for_each(|line| match line {
        Ok(line) => {
            let columns: Vec<_> = line.split('\t').collect();
            writeln!(output_f, "{}", columns[num]).unwrap();
            output_f.flush().expect("Error during flush");
        }
        Err(_) => panic!("parse error "),
    });
}


pub fn paste(input_file1: &Path, input_file2: &Path, output_file_name: &str) {
    let input_f1 = File::open(input_file1).expect("file not found");
    let input_f2 = File::open(input_file2).expect("file not found");
    let read_buf1 = BufReader::new(input_f1);
    let read_buf2 = BufReader::new(input_f2);
    let mut output_f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_name)
        .expect(format!("can't open file[{}] with write option", output_file_name).as_str());
    writeln!(output_f, "{}", read_buf1.lines().zip(read_buf2.lines()).map(|(col1, col2)| col1.unwrap() + "\t" + &col2.unwrap() + "\n").collect::<String>()).unwrap();
    output_f.flush().expect("Error during flush");
}

pub fn head(path: &Path, n: usize) -> io::Result<String> {
    let file = File::open(path)?;
    let read_buf = BufReader::new(file);
    read_buf.lines().take(n).map(|line| line.and_then(|line| Ok(line + "\n"))).collect()
}

pub fn tail(path: &Path, n: usize) -> io::Result<String> {
    let file = File::open(path)?;
    let read_buf = BufReader::new(file);
    let lines = read_buf.lines().collect::<Vec<_>>();
    let n_lines = lines.into_iter().rev().take(n).collect::<Vec<_>>();
    n_lines.into_iter().rev().map(|line| line.and_then(|line| Ok(line + "\n"))).collect()
}

pub fn split(path: &Path, n: usize) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let read_buf = BufReader::new(file);
    let lines = read_buf.lines().collect::<io::Result<Vec<_>>>();
    lines.and_then(|lines| Ok(lines.chunks(n).map(|chunk| chunk.join("\n")).collect()))
}


pub fn uniq(path: &Path, n: usize) -> HashSet<String> {
    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    let mut result = HashSet::new();
    br.lines().map(|line|
        line.and_then(|line|
            line.split_whitespace().map(|word| word.to_string()).nth(n)
            .ok_or(Error::new(ErrorKind::NotFound, format!("column not found.")))))
    .for_each(|line| match line {
        Ok(line) => { result.insert(line.to_string()); },
        Err(e) => eprintln!("{}", e)
    });
    result
}