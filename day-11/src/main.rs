use std::collections::{HashSet, VecDeque};
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

struct Octopii {
    octopii: Vec<Vec<u32>>
}

impl Octopii {

    fn height(&self) -> i32 {  self.octopii.len() as i32 }

    fn width(&self) -> i32 { self.octopii[0].len() as i32 }

    fn on_board(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >=0 && point.x < self.width() && point.y <  self.height()
    }

    // TODO(AE): write iterator
    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let dxdys = [(0,1), (0,-1), (1,0), (-1, 0), (1,1), (1,-1), (-1, 1), (-1,-1)];
        dxdys.iter().map(|dxdy| point.step(dxdy)).filter(|point|self.on_board(point)).collect()
    }

    fn next_step(&mut self) -> u32 {
        let mut next_step: Vec<Vec<u32>> = Vec::new();
        let mut flashed: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = VecDeque::new();
        (0..self.height()).for_each(|y|{
            next_step.push(Vec::new());
            (0..self.width()).for_each(|x|{
                next_step[y as usize].push(self.octopii[y as usize][x as usize] + 1)
            })
        });

        // TODO(AE): write iterator
        (0..self.height()).for_each(|y|{
            (0..self.width()).for_each(|x|{
                if next_step[y as usize][x as usize] > 9 { queue.push_back(Point { x, y }) }
            })
        });

        let mut flashes: u32 = 0;
        while !(queue.is_empty()) {
            let head = queue.pop_front().unwrap();
            if flashed.contains(&head) { continue }
            flashed.insert(head);
            flashes += 1;
            self.neighbours(&head).iter().for_each(|point| {
                let x_ = point.x as usize;
                let y_ = point.y as usize;
                next_step[y_][x_] = next_step[y_][x_] + 1;
                if next_step[y_][x_] > 9 {
                    queue.push_back(*point);
                }
            })
        }

        flashed.iter().for_each(|point| {
            next_step[point.y as usize][point.x as usize] = 0
        });

        self.octopii = next_step;

        flashes
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut octopii = read_lines(input).unwrap();
    let mut flashes: u32 = 0;
    for _t in 0..100 {
        flashes += octopii.next_step();
    }

    let mut needed = 0;
    let mut octopii = read_lines(input).unwrap();
    loop {
        flashes = octopii.next_step();
        needed += 1;
        if flashes == (octopii.octopii.len() * octopii.octopii[0].len()) as u32 { break; }
    }
    println!("{:?} is the first step during which all octopuses flash ..", needed);
    Ok(())

}

fn read_lines(filename: &String) -> io::Result<Octopii> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let octopii : Vec<Vec<u32>> = file_reader.lines()
                                             .map(|x|x.unwrap())
                                             .map(|x|x.chars().map(|x|x.to_digit(10).unwrap()).collect())
                                             .collect();
    Ok(Octopii { octopii })
}
