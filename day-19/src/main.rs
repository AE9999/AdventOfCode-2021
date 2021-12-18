use std::collections::HashMap;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use regex::Regex;

type BeaconDistance = f32;

#[derive(Debug)]
struct Problem {
    scanners: Vec<Scanner>
}

#[derive(Debug)]
struct Scanner {
    positions: Vec<Point>,
    // node_ids_2_distances : HashMap<usize, HashMap<usize, BeaconDistance>>
}

impl Scanner {

    fn new(positions: &Vec<Point>) -> Scanner {
        // let mut node_ids_2_distances: HashMap<usize, HashMap<usize, BeaconDistance>> = HashMap::new() ;
        // (0..positions.len()).for_each(|id|{
        //     node_ids_2_distances.insert(id, HashMap::new()) ;
        //     (0..positions.len()).for_each(|other_id|{
        //         let distance = positions[id].distance(&positions[other_id]);
        //         node_ids_2_distances.get_mut(&id).unwrap().insert(other_id, distance) ;
        //     })
        // });

        Scanner { positions: positions.clone() }
    }

    fn calculate_intersection(&self, other: &Scanner) {
        // For all possible transformations

        // For all possible offsets

        // Check which matches.
    }
}




#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
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

            scanners.push( Scanner::new(&positions));
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
    scanners.push( Scanner::new(&positions));
    Ok(Problem{ scanners })
}


