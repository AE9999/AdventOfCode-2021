use std::cmp::{min, max};
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use regex::Regex;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn step(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct TargetArea {
    lower_left: Point,
    upper_right: Point
}

struct Problem {
    target_area: TargetArea
}

impl Problem {
    fn in_target_area(&self, point: &Point) -> bool {
        point.x >= self.target_area.lower_left.x
        && point.x <= self.target_area.upper_right.x
        && point.y >= self.target_area.lower_left.y
        && point.y <= self.target_area.upper_right.y
    }

    fn past_target_area(&self, point: &Point) -> bool {
        point.x > self.target_area.upper_right.x
    }

    fn below_targat_area(&self, point: &Point) -> bool {
        point.y < self.target_area.lower_left.y
    }

    fn simiulate (&self, mut dx: i32, mut dy: i32) -> (bool, i32, Point) {
        let mut position = Point { x:0, y:0 };
        let mut maxy = i32::MIN;
        let mut hit_area = false;
        loop {
            position.step(dx, dy);
            maxy = max(maxy, position.y);
            hit_area |= self.in_target_area(&position);
            if self.past_target_area(&position) || self.below_targat_area(&position) {
                break
            }
            dx -= if dx == 0 { 0 }
            else { if dx > 0 { 1 }
            else { -1 } };
            dy -= 1;
        }
        (hit_area, maxy, position)
    }

    fn solve_part_one(&self) -> i32 {
        (1..self.target_area.upper_right.x+2).map(|dx|{
            let mut dy = 1;
            let mut global_max_y = i32::MIN;
            loop {
                let (hit, local_max_y, stop) = self.simiulate(dx, dy);
                if hit {
                    global_max_y = max(global_max_y, local_max_y)
                }
                if (self.past_target_area(&stop) || self.below_targat_area(&stop)) && !hit { break }
                dy += 1;
            }
            global_max_y
        }).max().unwrap()
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let problem = read_lines(input).unwrap();
    println!("{:?} is the highest y position it reaches on this trajectory ..", problem.solve_part_one());


    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;
    let  file_reader = BufReader::new(file_in);

    let re = Regex::new(r"target\sarea:\sx=(-?\d+)\.\.(-?\d+),\sy=(-?\d+)\.\.(-?\d+)").unwrap();
    let line = file_reader.lines().next().unwrap().unwrap();

    let cap = re.captures(line.as_ref()).unwrap();

    let xs: [i32; 2] = [ i32::from_str_radix(&cap[1], 10).unwrap(),
                         i32::from_str_radix(&cap[2], 10).unwrap()];
    let ys: [i32; 2] = [i32::from_str_radix(&cap[3], 10).unwrap(),
                        i32::from_str_radix(&cap[4], 10).unwrap()];

    let lower_left = Point { x: min(xs[0], xs[1]),
                             y: min(ys[0], ys[1]) };

    let upper_right = Point { x: max(xs[0], xs[1]),
                              y: max(ys[0], ys[1]) };

    let target_area = TargetArea { lower_left, upper_right };

    Ok(Problem { target_area })
}
