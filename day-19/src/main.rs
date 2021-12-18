use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use regex::Regex;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Problem {
    scanners: Vec<Scanner>
}

#[derive(Debug)]
struct Scanner {
    positions: Vec<Point>
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let problem = read_lines(input).unwrap();
    println!("{:?}", problem);
    Ok(())
}

fn read_lines(filename: &String,) -> io::Result<Problem> {
    let file_in = File::open(filename)?;
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut positions: Vec<Point> = Vec::new();
    let file_reader = BufReader::new(file_in);
    let re = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();
    file_reader.lines()
               .map(|line|line.unwrap())
               .for_each(|line|{
        if line.is_empty() {

            scanners.push( Scanner { positions });
            positions = Vec::new();
        }
        else if line.as_str().contains("scanne") {
            //
        }
        else {
            let cap = re.captures(line.as_ref()).unwrap();
            positions.push( Point {x: i32::from_str_radix(&cap[1], 10).unwrap(),
                                        y: i32::from_str_radix(&cap[1], 10).unwrap(),
                                        z: i32::from_str_radix(&cap[1], 10).unwrap()});
        }
    });
    scanners.push( Scanner { positions });
    Ok(Problem{ scanners })
}


