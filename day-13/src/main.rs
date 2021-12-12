use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::{HashSet, VecDeque};

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct Fold {
    axis: char,
    line: i32
}

#[derive(Debug)]
struct Origami {
    points: HashSet<Point>,
    folds: VecDeque<Fold>
}

impl Origami {

    fn height(&self) -> usize {
        (self.points.iter().map(|point|point.y).max().unwrap() + 1) as usize
    }

    fn widht(&self) -> usize {
        (self.points.iter().map(|point|point.x).max().unwrap() + 1) as usize
    }

    fn fold(&mut self) {
        let fold = self.folds.pop_front().unwrap();
        let next_points: HashSet<Point> = self.points.iter().map(|point| {
            if fold.axis == 'x' {
                let newx = if point.x < fold.line { point.x }
                                else { fold.line - (point.x - fold.line) };
                Point { x:  newx, y: point.y }
            }
            else {
                let newy = if point.y < fold.line { point.y }
                                else { fold.line - (point.y - fold.line) };
                Point { x: point.x, y: newy }
            }
        }).collect();
        self.points = next_points;
    }

    fn print(&self) {
        let mut output: Vec<Vec<char>> = (0..self.height()).map(|_x|vec!['.'; self.widht()]).collect();
        self.points.iter().for_each(|point|output[point.y as usize][point.x as usize] = '#');
        output.iter().for_each(|line| println!("{:?}", line.iter().collect::<String>()));
    }
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut origami = read_lines(input).unwrap();
    origami.fold();
    println!("{:?} many dots are visible after completing just the first fold instruction on your transparent paper ..",
             origami.points.len());
    while !origami.folds.is_empty() {
        origami.fold();
    }
    origami.print();
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Origami> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let lines: Vec<String> = file_reader.lines().map(|x| x.unwrap()).collect();
    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: VecDeque<Fold> = VecDeque::new();
    let mut reading_points = true;
    lines.iter().for_each(|line| {
        if line.is_empty() {
            reading_points = false;
        } else {
            if line.starts_with("fold along ") {
                let mut split =  line["fold along ".len()..].split("=");
                folds.push_back(Fold {
                    axis: split.next().unwrap().chars().next().unwrap(),
                    line: split.next().unwrap().parse::<i32>().unwrap()
                })
            } else {
                let mut split = line.split(',');
                points.insert(Point { x: split.next().unwrap().parse::<i32>().unwrap(),
                                         y: split.next().unwrap().parse::<i32>().unwrap() } );
            }
        }

    });
    Ok(Origami { points, folds })
}
