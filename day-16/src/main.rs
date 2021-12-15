use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    read_lines(input);
}

fn read_lines(filename: &String) -> io::Result<Vec<String>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

