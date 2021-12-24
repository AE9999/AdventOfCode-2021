use std::collections::HashMap;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use lazy_static::lazy_static; // 1.3.0
use regex::Regex; // 1.1.5

lazy_static! {
    static ref RE: Regex = Regex::new(r"(-?\d+)").unwrap();
}

struct Program {
    instructions: Vec<String>,
    pc: usize,
    memory : HashMap<char, i64>
}

impl Program {
    fn new(instructions: Vec<String>) -> Program {
        let mut memory: HashMap<char, i64> = HashMap::new();
        memory.insert('w', 0);
        memory.insert('x', 0);
        memory.insert('y', 0);
        memory.insert('z', 0);

        Program {
            instructions,
            pc: 0,
            memory
        }
    }

    fn value(&self, value: &str) -> i64 {
        if RE.is_match(value) {
            let cap = RE.captures(value).unwrap();
            i64::from_str_radix(&cap[0], 10).unwrap()
        } else {
            let address: char = value.parse().unwrap();
            *(self.memory.get(&address).unwrap())
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.memory.insert('w', 0);
        self.memory.insert('x', 0);
        self.memory.insert('y', 0);
        self.memory.insert('z', 0);
    }

    fn run(&mut self, input: Vec<char>) -> i64 {
        self.reset();
        let mut ic = 0;
        while self.pc < self.instructions.len() {
            let instruction: Vec<&str> = self.instructions[self.pc].split(' ').collect();
            match instruction[0] {
                "inp" => {
                    let address: char = instruction[1].parse().unwrap();
                    self.memory.insert(address, input[ic].to_digit(10).unwrap() as i64);
                    ic += 1
                },
                "add" => {
                    let address: char = instruction[1].parse().unwrap();
                    let value = self.value(instruction[2]);
                    self.memory.insert(address, self.memory.get(&address).unwrap() + value);
                },
                "mul" => {
                    let address: char = instruction[1].parse().unwrap();
                    let value = self.value(instruction[2]);
                    self.memory.insert(address, self.memory.get(&address).unwrap() * value);
                },
                "div" => {
                    let address: char = instruction[1].parse().unwrap();
                    let value = self.value(instruction[2]);
                    self.memory.insert(address, self.memory.get(&address).unwrap() / value);
                },
                "mod" => {
                    let address: char = instruction[1].parse().unwrap();
                    let value = self.value(instruction[2]);
                    self.memory.insert(address, self.memory.get(&address).unwrap() % value);
                },
                "eql" => {
                    let address: char = instruction[1].parse().unwrap();
                    let value_right = self.value(instruction[2]);
                    let value_left = *(self.memory.get(&address).unwrap());
                    let value = if value_left == value_right { 1 } else { 0 } as i64;
                    self.memory.insert(address,  value);
                },
                _ => panic!("illegal instruction")
            }
            self.pc += 1
        }
        *(self.memory.get(&'z').unwrap())
    }
}

fn solve1(buffer: &mut Vec<char>,
          program: &mut Program) -> i64 {
    if buffer.len() == 14 {
        let input = String::from_iter(buffer.iter());
        let result  = program.run(buffer.clone());
        // println!("Testing {:?}", input);
        println!("{:?} => {:?} ", input, result);
        if result == 1 {
            return i64::from_str_radix(input.as_str(), 10).unwrap()
        }
        return -1
    } else {
        for c in  (1..10).rev() {
            let char = char::from_digit(c, 10).unwrap();
            buffer.push(char);
            let answer = solve1(buffer, program);
            if answer > 0 { return answer }
            buffer.pop();
        }
        return -1
    }
}

fn main()  -> io::Result<()> {
    let mut program = read_lines(&env::args().collect::<Vec<String>>()[1]).unwrap();
    let mut buffer: Vec<char> = Vec::new();
    //println!("{:?} is the largest model number accepted by MONAD", solve1(&mut buffer, &mut program));
    println!("{:?}", program.run("11111111111111".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("91111111111111".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("11111111111119".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("11111111111191".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("11111111111911".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("11111111119111".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("11111111191111".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("11111111181111".chars().collect::<Vec<char>>()));
    /*println!("{:?}", program.run("22222222222222".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("33333333333333".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("12345678912345".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("54321987654321".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("11111111222222".chars().collect::<Vec<char>>()));
    println!("{:?}", program.run("22222221111111".chars().collect::<Vec<char>>()));*/
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Program> {

    let lines = BufReader::new(File::open(filename)?).lines().map(|line|{
        line.unwrap()
    }).collect::<Vec<String>>();

    Ok((Program::new(lines)))
}
