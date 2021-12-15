use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

#[derive(Debug)]
struct Problem {
    polymer: Vec<char>,

    pair_insertion_rules: HashMap<(char, char), char>,

    calculated_pair_insertion_rules: HashMap<(char, char), HashMap<char, u64>>,

    //calculated_pair_insertion_sub: HashMap<(char, char), Vec<char>>
}

impl Problem {

    fn do_step(&self, input: &Vec<char>) -> Vec<char> {
        let mut next_polymer: Vec<char> = Vec::new();

        (1..input.len()).for_each(|i| {
            let l = input[i-1];
            let r = input[i];
            next_polymer.push(l);
            next_polymer.push(self.pair_insertion_rules.get(&(l,r) ).unwrap().clone());
        });
        next_polymer.push(input.last().unwrap().clone());
        return next_polymer
    }

    fn step(&mut self) {
        self.polymer = self.do_step(&self.polymer);
    }

    fn calculate_answer(&self) -> u64 {
        let mut rvalue: HashMap<char, u64> = HashMap::new();
        self.polymer.iter().for_each(|x| *rvalue.entry(*x).or_insert(0) += 1);
        let max = rvalue.iter().map(|(_k,v)|v).max().unwrap();
        let min = rvalue.iter().map(|(_k,v)|v).min().unwrap();
        max - min
    }

    fn calculate_pair_insertion_rules(&mut self, amount: usize) {

        for (k, _v) in self.pair_insertion_rules.iter() {
            let mut input: Vec<char> = Vec::new();
            input.push(k.0);
            input.push(k.1);
            (0..amount).for_each(|_x| input = self.do_step(&input));
            let mut map : HashMap<char, u64> = HashMap::new();
            input[1..input.len()-1].iter().for_each(|c|{
                *map.entry(*c).or_insert(0) += 1
            });
            self.calculated_pair_insertion_rules.insert(*k, map);
        }
    }

    fn calculate_final_score(&mut self) -> u64 {
        let mut map : HashMap<char, u64> = HashMap::new();

        self.polymer.iter().for_each(|c|{
            *map.entry(*c).or_insert(0) += 1
        });
        (1..self.polymer.len()).for_each(|i| {
            let l = self.polymer[i - 1];
            let r = self.polymer[i];


            self.calculated_pair_insertion_rules.get(&(l,r))
                .unwrap()
                .iter()
                .for_each(|(k,v)| {
                    map.insert(*k, map.get(k).unwrap() + *v);
                })
        });
        let max = map.iter().map(|(_k,v)|v).max().unwrap();
        let min = map.iter().map(|(_k,v)|v).min().unwrap();
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

    let mut problem = read_lines(input).unwrap();
    (0..20).for_each(|_x| problem.step());
    problem.calculate_pair_insertion_rules(20);
    println!("{:?} is what you get if you take the quantity of the most common element and subtract the quantity of the least common element", problem.calculate_final_score());

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
    Ok(Problem { polymer,
                 pair_insertion_rules,
                 calculated_pair_insertion_rules: HashMap::new() } )
}
