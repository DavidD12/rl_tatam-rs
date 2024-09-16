use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn lines_from_file(path: &String) -> Vec<String> {
    if path.is_empty() {
        Vec::new()
    } else {
        let file = File::open(path).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }
}