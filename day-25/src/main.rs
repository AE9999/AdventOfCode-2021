use std::collections::HashSet;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;

struct Problem {
    cucumbers: Vec<Vec<char>>,
}

impl Problem {

    fn new(cucumbers: Vec<Vec<char>>) -> Problem {
        Problem {
            cucumbers
        }
    }

    fn height(&self) -> i32 { self.cucumbers.len() as i32 }

    fn width(&self) -> i32  { self.cucumbers[0].len() as i32 }

    fn value(&self, point: &Point) -> char {
        self.cucumbers[point.y as usize][point.x as usize]
    }

    fn update(&mut self, my_move: &Move) {
        let value = self.cucumbers[my_move.origin.y as usize][my_move.origin.x as usize];
        self.cucumbers[my_move.origin.y as usize][my_move.origin.x as usize] = '.';
        self.cucumbers[my_move.destination.y as usize][my_move.destination.x as usize] = value;
    }

    fn print(&self) {
        println!("{:?}", String::from_iter(vec!['!'; self.width() as usize]));
        self.cucumbers.iter().for_each(|x| {
            println!("{:?}", x.iter().collect::<String>());
        });
        println!("{:?}", String::from_iter(vec!['!'; self.width() as usize]));
    }

    fn step(&mut self) -> usize {
        let mut points_moved: usize = 0;
        let mut moves: Vec<Move> = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let origin = Point::new(x,y);
                match self.value(&origin) {
                    'v' => { continue }
                    '>' => {
                        let x_ = (x + 1) % self.width();
                        let destination = Point::new(x_, y);
                        if self.value(&destination) == '.' {
                            moves.push(Move::new(origin, destination));
                        }
                    }
                    '.' => {  continue }
                    _ => panic!("")
                }
            }
        }

        moves.iter().for_each(|my_move| self.update(my_move));
        points_moved += moves.len();
        moves.clear();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let origin = Point::new(x,y);
                match self.value(&origin) {
                    '>' => { continue }
                    'v' => {
                        let y_ = (y + 1) % self.height();
                        let destination = Point::new(x, y_);
                        if self.value(&destination) == '.' {
                            moves.push(Move::new(origin, destination));
                        }
                    }
                    '.' => {  continue }
                    _ => panic!("")
                }
            }
        }

        moves.iter().for_each(|my_move| self.update(my_move));
        points_moved += moves.len();
        moves.clear();

        points_moved
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {

    fn new(x: i32, y: i32) -> Point {
        Point {x,y}
    }

}

struct Move {
    origin: Point,
    destination: Point
}

impl Move {
    fn new(origin: Point, destination: Point) -> Move {
        Move {
            origin,
            destination
        }
    }
}

fn main() -> io::Result<()> {
    let mut problem: Problem = read_lines(&env::args().collect::<Vec<String>>()[1]).unwrap();
    let mut steps: u32 = 0;
    // problem.print();
    loop {
        steps += 1;
        // println!("After step: {:?}", steps);
        let amount_moved = problem.step();
        // problem.print();
        if amount_moved == 0 { break; }
    }
    println!("{:?} is the first step on which no sea cucumbers move ..", steps);
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let cucumbers: Vec<Vec<char>> = BufReader::new(File::open(filename)?).lines().map(|line|{
        line.unwrap().chars().collect::<Vec<char>>()
    }).collect();
    Ok(Problem::new(cucumbers))
}
