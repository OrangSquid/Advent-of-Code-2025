use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

// We return an io::Result to handle errors safely (instead of unwrap)
pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(Result::unwrap).collect()
}