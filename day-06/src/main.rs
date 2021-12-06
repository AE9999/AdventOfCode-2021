use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let mut states: HashMap<i32, u64> = (0..9).map(|i|(i,0)).collect();
    read_input(input).unwrap().iter().for_each(|x| {
        let amount = states.get(x).unwrap() + 1;
        states.insert(*x, amount);
    });

    for _ in 0..80 {
        update(&mut states);
    };
    let amount = states.iter().map(|(_key,value)|value).fold(0, |x, acc| acc +x);
    println!("After 80 days we have {:?} lanternfish ..", amount);


    let mut states: HashMap<i32, u64> = (0..9).map(|i|(i,0)).collect();
    read_input(input).unwrap().iter().for_each(|x| {
        let amount = states.get(x).unwrap() + 1;
        states.insert(*x, amount);
    });

    for _ in 0..256 {
        update(&mut states);
    };
    let amount = states.iter().map(|(_key,value)|value).fold(0, |x, acc| acc +x);
    println!("After 256 days we have {:?} lanternfish ..", amount);
    Ok(())
}

fn update(states: &mut HashMap<i32, u64>) {
    let mut next_states: HashMap<i32, u64> = (0..states.len()).map(|i|(i as i32 ,0u64)).collect();
    for counter in (0..states.len() as i32) {
        let amount = states.get(&counter).unwrap();
        if counter == 0 {
            next_states.insert(6, *amount);
            next_states.insert(8, *amount);
        } else {
            let nvalue = next_states.get(&(counter - 1)).unwrap() + amount;
            next_states.insert(counter - 1, nvalue);
        }
    }

    for i in (0..states.len() as i32) {
        states.insert(i, *next_states.get(&i).unwrap());
    }
}

fn read_input(filename: &String) ->  io::Result<Vec<i32>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().next().unwrap().unwrap().split(",").map(|x|x.parse::<i32>().unwrap()).collect())
}
