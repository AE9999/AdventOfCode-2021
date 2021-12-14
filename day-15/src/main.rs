use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use std::collections::{HashSet, VecDeque};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    read_lines(input);
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Origami> {
    let file_in = File::open(filename) ?;
    let file_reader = BufReader::new(file_in);
    return Ok(file_reader.lines().map(|x| x.unwrap()))
}
