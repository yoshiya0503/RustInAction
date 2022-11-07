use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn read1() {
    let f = File::open("../README.md").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);
        line.truncate(0);
    }
}

fn read2() {
    let f = File::open("../README.md").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let result = line.unwrap();
        println!("{} ({} bytes long)", result, result.len());
    }
}

fn main() {
    read1();
    read2();
}
