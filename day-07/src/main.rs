use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let crabs = read_input(input).unwrap();
    let min_pos: i32 = *crabs.iter().map(|(k, _v)| k).min().unwrap();
    let max_pos: i32 = *crabs.iter().map(|(k, _v)| k).max().unwrap() + 1;

    let mut candidates : Vec<(i32, i32)> = (min_pos..max_pos).map(|current_entry| {
        crabs.iter()
             .map(|(k,v)| (k - current_entry).abs() * v)
             .fold((current_entry, 0), |acc,x|  (acc.0, acc.1 + x) )
    }).collect();
    candidates.sort_by_key(|x|x.1);

    println!("{:?} is best and {:?} fuel must be  spend to align to that position ..",
             candidates.first().unwrap().0,
             candidates.first().unwrap().1);

    let mut candidates : Vec<(i32, i32)> = (min_pos..max_pos).map(|current_entry| {
        crabs.iter()
             .map(|(k,v)| euler((k - current_entry).abs()) * v)
             .fold((current_entry, 0), |acc,x|  (acc.0, acc.1 + x) )
    }).collect();
    candidates.sort_by_key(|x|x.1);

    println!("{:?} is best and actually {:?} fuel must be  spend to align to that position ..",
             candidates.first().unwrap().0,
             candidates.first().unwrap().1);

    Ok(())
}

fn euler(x: i32) -> i32 {  (x + 1) * x / 2 }

fn read_input(filename: &String) ->  io::Result<HashMap<i32, i32>> {
    let mut rvalue: HashMap<i32, i32> = HashMap::new();
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let crabs : Vec<i32> = file_reader.lines().next().unwrap().unwrap().split(",").map(|x|x.parse::<i32>().unwrap()).collect();
    crabs.iter().for_each(|x| *rvalue.entry(*x).or_insert(0) += 1);
    Ok(rvalue)
}
