use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn step(&self, dxdy: &(i32, i32)) -> Point {
        Point { x: self.x + dxdy.0, y: self.y + dxdy.1 }
    }
}

struct HeightMap {
    board: Vec<Vec<u32>>
}

// 1508: Too high

impl HeightMap {

    fn height(&self) -> i32 {  self.board.len() as i32 }

    fn width(&self) -> i32 { self.board[0].len() as i32 }

    fn value(&self, point: & Point) -> u32 {
        self.board[point.y as usize][point.x as usize]
    }

    fn on_board(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >=0 && point.x < self.width() && point.y <  self.height()
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let dxdys = [(0,1), (0,-1), (1,0), (-1, 0)];
        dxdys.iter().map(|dxdy| point.step(dxdy)).filter(|point|self.on_board(point)).collect()
    }

    fn low_points(&self) -> Vec<Point> {
        let mut rvalue: Vec<Point> = Vec::new();

        (0..self.width()).for_each(|x|
            { (0..self.height()).for_each(|y| {
                let point = Point { x, y };
                let counter = self.neighbours(&point)
                                        .iter()
                                        .find(|n|self.value(&point) >= self.value(n))
                                        .is_none();
                if counter {
                    rvalue.push(point)
                }

            }) }
        );
        rvalue
    }

    fn measure_basin(&self, low_point: &Point) -> i32 {
        let mut basin: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = VecDeque::new();
        queue.push_back(*low_point);
        while !queue.is_empty() {
            let point = queue.pop_front().unwrap();
            if !(basin.contains(&point)) {
                basin.insert(point);
                self.neighbours(&point)
                    .iter()
                    .filter(|p| self.value(p) < 9).for_each(|p|queue.push_back(*p))
            }
        }
        basin.len() as i32
    }
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let height_map = input2height_map(input).unwrap();
    let answer = height_map.low_points()
                                .iter()
                                .map(|x|height_map.value(x) + 1)
                                .fold(0, |acc,x| acc + x);
    println!("{:?} is the sum of the risk levels of all low points on your heightmap ..", answer);
    let mut basin_sizes: Vec<i32> =  height_map.low_points()
                                           .iter()
                                           .map(|x|height_map.measure_basin(x))
                                           .collect();
    basin_sizes.sort();
    let answer = basin_sizes.iter().rev().take(3).fold(1, |acc,x|acc * x);
    println!("{:?} is what you get if you multiply together the sizes of the three largest basins ..", answer);
    Ok(())
}

fn input2height_map(filename: &String) -> io::Result<HeightMap> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let board: Vec<Vec<u32>> = file_reader.lines()
                                          .filter_map(io::Result::ok)
                                          .map(|line| line.as_str().chars().map(|c|c.to_digit(10).unwrap()).collect())
                                          .collect();
    return Ok(HeightMap { board })
}
