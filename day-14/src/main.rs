use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

// Stolen from https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676 and
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c3d261e77626ce7f0c7de8d75547b866

#[derive(Clone, Debug)]
enum PolymerNode {
    None,
    Tail { item: char },
    Link { item: char, next: Box<PolymerNode> }
}

impl PolymerNode {

    fn new() -> Self {
        Self::None
    }

    fn value(&self) -> Option<char> {
        match self {
            Self::None => return None,
            Self::Link { item, next: _ } => return Option::from(*item),
            Self::Tail { item } => return Option::from(*item)
        }
    }

    fn push(&mut self, item: char) -> &mut PolymerNode {
        match self {
            Self::None => {
                self.to_tail(item);
                return self
            },
            Self::Tail { .. } => {
                self.to_link(item);

                match self {
                    Self::Link { item, next} => {  return next.as_mut() }
                    _ => panic!("Something else"),
                }
            },
            _ => panic!("Inefficient.")
        };
    }

    fn to_tail(&mut self, item: char) {
        *self = match self {
            Self::None => Self::Tail { item },
            Self::Link { item:_, next:_ } => Self::Tail { item },
            _ => panic!("Supplied value was not of correct type or variant.")
        }
    }

    fn to_link(&mut self, x: char) {
        *self = match self {
                Self::Tail { item } => {
                Self::Link { item: *item, next: Box::new(Self::Tail { item: x }) }
            },
            _ => { panic!("something went wrong"); }
        };
    }

    fn insert_next(&mut self, x: char) -> &mut PolymerNode {
        match self {
            Self::Link { item, next} => {
                *self = Self::Link { item: *item,
                                     next: Box::new(Self::Link { item: x, next: next.to_owned() }) }
            }
            _ => panic!("Cannot be called on non link node")
        }
        self.next().unwrap().next().unwrap() // Yeah I'm not proud of it
    }

    fn insert_at(&mut self, x: char) -> &mut PolymerNode {
        match self {
            Self::Link { item, next} => {
                *self = Self::Link { item: *item,
                    next: Box::new(Self::Link { item: x, next: next.to_owned() }) }
            }
            _ => panic!("Cannot be called on non link node")
        }
        self.next().unwrap() // Still definitly not proud
    }

    fn next(&mut self) -> Option<&mut Box<PolymerNode>> {
        match self {
            Self::None => return None,
            Self::Tail { item } => return None,
            Self::Link { item, next} => {  return Option::from(next) }
        }
    }

    fn read_next(&self) -> Option<&Box<PolymerNode>> {
        match self {
            Self::None => return None,
            Self::Tail { item } => return None,
            Self::Link { item, next} => {  return Option::from(next) }
        }
    }

    fn pairs(&self) -> Option<(char, char)> {
        match self {
            Self::None => return None,
            Self::Tail { item } => return None,
            Self::Link { item, next  } => return Option::from((self.value().unwrap(),
                                                                                           next.value().unwrap()))
        }
    }
}

#[derive(Debug)]
struct Problem {
    polymer_node: PolymerNode,

    pair_insertion_rules: HashMap<(char, char), char>
}

impl Problem {

    fn state(&self) -> String {
        let mut rvalue = String::new();
        let mut head = self.polymer_node.borrow();
        let mut value = head.value();
        while value.is_some() {
            rvalue.push(value.unwrap());
            let head_ = head.read_next();
            if head_.is_some() {
                head = head_.unwrap().as_ref();
                value = head.value();
            } else {
                value = None;
            }
        }
        rvalue
    }

    fn step(&mut self) {
        let mut head = self.polymer_node.borrow_mut();
        let mut pairs = head.pairs();
        while pairs.is_some() {
            let key = pairs.unwrap();
            let insertion = self.pair_insertion_rules.get(&key).unwrap();
            head = head.insert_next(*insertion);
            pairs = head.pairs();
        }
        println!("{:?}", self.state())
    }

    fn step_precalculated(&mut self, pre_calculated_rules: &HashMap<(char, char), String>) {
        let mut head = self.polymer_node.borrow_mut();
        let mut pairs = head.pairs();
        while pairs.is_some() {
            let key = pairs.unwrap();
            let insertion = pre_calculated_rules.get(&key).unwrap();
            for c in insertion.chars() {
                head = head.insert_at(c);
            }
            head = head.next().unwrap();
            pairs = head.pairs();
        }
        println!("{:?}", self.state())
    }
}

fn calculate_answer(state: String) -> u64 {
    let mut rvalue: HashMap<char, u64> = HashMap::new();
    state.chars().for_each(|x| *rvalue.entry(x).or_insert(0) += 1);
    let max = rvalue.iter().map(|(_k,v)|v).max().unwrap();
    let min = rvalue.iter().map(|(_k,v)|v).min().unwrap();
    max - min
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut problem = read_lines(input).unwrap();
    (0..2).for_each(|_x| problem.step());
    (0..2).for_each(|_x| problem.step());
    let state = problem.state();
    let answer = calculate_answer(state);
    println!("{:?} is what you get if you take the quantity of the most common element and subtract the quantity of the least common element", answer);

    let mut problem = read_lines(input).unwrap();
    let mut help_map: HashMap<(char, char), String> = HashMap::new();
    problem.pair_insertion_rules.iter().for_each(|(k,v_)| {
        let mut node = PolymerNode::new();
        let mut node_ = node.borrow_mut();
        node_ = node_.push((k.0));
        node_ = node_.push((k.1));
        let mut problem_ = Problem {
            pair_insertion_rules: problem.pair_insertion_rules.clone(),
            polymer_node: node
        };
        (0..2).for_each(|_x| problem_.step());
        // We need to trip out the first and last char because it is already there
        let complet_state = problem_.state();
        let state = String::from(&complet_state[1..complet_state.len() - 1]);
        // println!("{:?} => {:?} ..", k, complet_state);
        help_map.insert(*k, state);
    });

    problem.step_precalculated(&help_map);
    problem.step_precalculated(&help_map);
    let state = problem.state();
    let answer = calculate_answer(state);
    println!("{:?} is what you get if you take the quantity of the most common element and subtract the quantity of the least common element", answer);

    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let mut polymer_template = PolymerNode::new();
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
            let mut tail = polymer_template.borrow_mut();
            for c in line.chars() {
                tail = tail.push(c);
            }
        }
    });
    Ok(Problem { polymer_node: polymer_template, pair_insertion_rules } )
}
