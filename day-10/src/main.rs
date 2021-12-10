use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

// https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html
lazy_static! {
    static ref OPEN2CLOSE: HashMap<char, char> = {
        let mut map = HashMap::new();
        map.insert('(', ')');
        map.insert('{', '}');
        map.insert('[', ']');
        map.insert('<', '>');
        map
    };
}

lazy_static! {
    static ref CLOSE2SCORE: HashMap<char, i64> = {
        let mut map = HashMap::new();
        map.insert(')', 3);
        map.insert('}', 1197);
        map.insert(']', 57);
        map.insert('>', 25137);
        map
    };
}

lazy_static! {
    static ref AUTOCOMPLETE2SCORE: HashMap<char, i64> = {
        let mut map = HashMap::new();
        map.insert(')', 1);
        map.insert(']', 2);
        map.insert('}', 3);
        map.insert('>', 4);
        map
    };
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let lines = read_lines(input).unwrap();
    let answer = lines.iter().map(|line| check_for_errors(line.as_str())).fold(0, |acc, x| acc + x);
    println!("{:?} is the total syntax error score for those errors", answer);
    let mut autocomplet_scores: Vec<i64> =  lines.iter()
                                                 .map(|line| check_for_auto_complete(line.as_str()))
                                                 .filter(|x| *x != -1)
                                                 .collect();
    autocomplet_scores.sort();
    let answer = autocomplet_scores[(autocomplet_scores.len() / 2)];
    println!("{:?} is the middle score", answer);
    Ok(())
}

fn is_open(char: &char) -> bool { OPEN2CLOSE.contains_key(char) }

fn closes(open: &char, close: &char) -> bool { OPEN2CLOSE.get(open).unwrap() == close }

fn check_for_errors(line: &str) -> i64 {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_open(&c) { stack.push(c) } else {
            let open = stack.pop().unwrap();
            if !closes(&open, &c) { return *CLOSE2SCORE.get(&c).unwrap() }
        }
    };
    0
}

fn check_for_auto_complete(line: &str) -> i64 {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_open(&c) { stack.push(c) } else {
            let open = stack.pop().unwrap();
            if !closes(&open, &c) { return -1 }
        }
    };
    if stack.is_empty() { -1 }
    else {
        stack.iter()
             .rev()
             .map(|c| {
                let close = OPEN2CLOSE.get(c).unwrap();
                 AUTOCOMPLETE2SCORE.get(close).unwrap()
             })
             .fold(0, |acc,x| (acc *5) + x)
    }
}


fn read_lines(filename: &String) -> io::Result<Vec<String>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
