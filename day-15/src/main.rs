use std::cmp::Ordering;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use std::collections::{HashMap};
use std::collections::BinaryHeap;


#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering { other.y.cmp(&self.y).then_with(|| self.x.cmp(&other.x)) }
}

impl Point {
    fn step(&self, dxdy: &(i32, i32)) -> Point {
        Point { x: self.x + dxdy.0, y: self.y + dxdy.1 }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
                  .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct RiskMap {
    points: Vec<Vec<u32>>
}

impl RiskMap {
    fn height(&self) -> i32 {  self.points.len() as i32 }

    fn width(&self) -> i32 { self.points[0].len() as i32 }

    fn on_board(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >=0 && point.x < self.width() && point.y <  self.height()
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let dxdys = [(0,1), (0,-1), (1,0), (-1, 0)];
        dxdys.iter().map(|dxdy| point.step(dxdy)).filter(|point|self.on_board(point)).collect()
    }

    fn risk(&self, point: Point) -> u32 { self.points[point.y as usize][point.x as usize] }

    // Stolen from https://doc.rust-lang.org/std/collections/binary_heap/index.html
    fn calculate_risk(&self) -> u32 {
        let mut dist: HashMap<Point, u32> = HashMap::new();
        (0..self.height()).for_each(|y| {
            (0..self.width()).for_each(|x| {
                dist.insert(Point{x ,y }, u32::MAX);
            })
        });

        let mut heap : BinaryHeap<State> = BinaryHeap::new();
        let goal = Point { x: self.width() - 1, y: self.height() - 1 };

        dist.insert(Point {x: 0, y: 0 }, 0);
        heap.push(State { cost: 0, position: Point {x: 0, y: 0 } });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal { return cost ; }
            if cost > dist[&position] { continue; }

            self.neighbours(&position).iter().map(|neighbour|{
                State { cost: cost + self.risk(*neighbour), position: neighbour.clone() }
            }).for_each(|next| {
                if next.cost < dist[&next.position] {
                    heap.push(next);
                    dist.insert(next.position, next.cost);
                }
            })

        }
        panic!("End state should be reachable");
    }

    fn height_extended(&self) -> i32 { (self.points.len() * 5) as i32 }

    fn width_extended(&self) -> i32 { (self.points[0].len() * 5) as i32 }

    fn on_board_extended(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >=0 && point.x < self.width_extended() && point.y <  self.height_extended()
    }

    fn neighbours_extended(&self, point: &Point) -> Vec<Point> {
        let dxdys = [(0,1), (0,-1), (1,0), (-1, 0)];
        dxdys.iter().map(|dxdy| point.step(dxdy))
                    .filter(|point|self.on_board_extended(point))
                    .collect()
    }

    fn risk_extended(&self, point: Point) -> u32 {
        let timesx = (point.x as u32) / (self.width() as u32);
        let timesy: u32 = (point.y as u32) / (self.height() as u32);

        let y = (point.y % self.height()) as usize;
        let x = (point.x % self.width()) as usize;

        let result = self.points[y][x] + timesx + timesy;
        if result > 9 {
            1 + (result % 10)
        } else {
            result
        }

    }

    fn calculate_risk_extended(&self) -> u32 {
        let mut dist: HashMap<Point, u32> = HashMap::new();
        (0..self.height_extended()).for_each(|y| {
            (0..self.width_extended()).for_each(|x| {
                dist.insert(Point{x ,y }, u32::MAX);
            })
        });

        let mut heap : BinaryHeap<State> = BinaryHeap::new();
        let goal = Point { x: self.width_extended() - 1, y: self.height_extended() - 1 };

        dist.insert(Point {x: 0, y: 0 }, 0);
        heap.push(State { cost: 0, position: Point {x: 0, y: 0 } });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal { return cost ; }
            if cost > dist[&position] { continue; }

            self.neighbours_extended(&position).iter().map(|neighbour|{
                State { cost: cost + self.risk_extended(*neighbour), position: neighbour.clone() }
            }).for_each(|next| {
                if next.cost < dist[&next.position] {
                    heap.push(next);
                    dist.insert(next.position, next.cost);
                }
            })

        }
        panic!("End state should be reachable");
    }

}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let risk_map = read_lines(input).unwrap();
    println!("{:?} is the lowest total risk of any path from the top left to the bottom right?",
             risk_map.calculate_risk());
    println!("{:?} is the lowest total risk of any path from the top left to the bottom right?",
             risk_map.calculate_risk_extended());

    Ok(())
}

fn read_lines(filename: &String) -> io::Result<RiskMap> {

    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let points : Vec<Vec<u32>> = file_reader.lines()
                                             .map(|x|x.unwrap())
                                             .map(|x|x.chars().map(|x|x.to_digit(10).unwrap()).collect())
                                             .collect();
    Ok(RiskMap{
        points
    })
}
