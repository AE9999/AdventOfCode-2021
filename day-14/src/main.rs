use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

#[derive(Debug)]
struct Problem {
    polymer: Vec<char>,

    pair_insertion_rules: HashMap<(char, char), char>
}

impl Problem {

    fn step(&mut self) {
        println!("Step ..");
        let mut next_polymer: Vec<char> = Vec::new();
        (1..self.polymer.len()).for_each(|i| {
            let l = self.polymer[i-1];
            let r = self.polymer[i];
            next_polymer.push(l);
            next_polymer.push(self.pair_insertion_rules.get(&(l,r) ).unwrap().clone());
        });
        next_polymer.push(self.polymer.last().unwrap().clone());
        self.polymer = next_polymer;
    }

    fn calculate_answer(&self) -> u64 {
        let mut rvalue: HashMap<char, u64> = HashMap::new();
        self.polymer.iter().for_each(|x| *rvalue.entry(*x).or_insert(0) += 1);
        let max = rvalue.iter().map(|(_k,v)|v).max().unwrap();
        let min = rvalue.iter().map(|(_k,v)|v).min().unwrap();
        max - min
    }

}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut problem = read_lines(input).unwrap();
    (0..10).for_each(|_x| problem.step());
    println!("{:?} is what you get if you take the quantity of the most common element and subtract the quantity of the least common element",
            problem.calculate_answer());
    (0..30).for_each(|_x| problem.step());
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let mut polymer: Vec<char> = Vec::new();
    let mut pair_insertion_rules: HashMap<(char, char), char> = HashMap::new();

    file_reader.lines().map(|x| x.unwrap()).for_each(|line| {
        if line.is_empty() { return }
        if line.contains("->") {
            let mut split = line.split(" -> ");
            let mut antecedent = split.next().unwrap().chars();
            let mut result = split.next().unwrap().chars();
            pair_insertion_rules.insert((antecedent.next().unwrap(),
                                            antecedent.next().unwrap()),

                                     result.next().unwrap());
        } else {
            line.chars().for_each(|c| polymer.push(c))
        }
    });
    Ok(Problem { polymer, pair_insertion_rules } )
}
