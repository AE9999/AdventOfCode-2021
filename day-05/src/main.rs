use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::ops::Range;

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
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn points_straight(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            straigt_endpoints_to_interval(self.start.y, self.end.y).map(|y|Point {x:self.start.x, y}).collect()
        }  else {
            straigt_endpoints_to_interval(self.start.x, self.end.x).map(|x|Point {x, y:self.start.y}).collect()
        }
    }

    fn points(&self) -> Vec<Point> {
        if self.straight() {
            return self.points_straight();
        }
        let (left, right) = if self.start.x < self.end.x { (self.start, self.end) } else { (self.end, self.start) };
        let dy = if left.y > right.y { -1 } else { 1 };
        let amount = max (self.start.x, self.end.x) - min (self.start.x, self.end.x) + 1;
        (0..amount).map(|i| Point { x: left.x + i, y: left.y + (dy * i)   } ).collect()
    }
}

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let mut straight_lines: Vec<Line> = read_input(input).unwrap();
    straight_lines.retain(|line| line.straight());


    let mut point2lines: HashMap<Point, Vec<&Line>> = HashMap::new();
    straight_lines.iter().for_each(|line| {
                  let points = line.points_straight();
                  points.iter()
                        .for_each(|point| {
                          if !point2lines.contains_key(point) {
                            // https://users.rust-lang.org/t/creates-a-temporary-which-is-freed-while-still-in-use-again/29211
                            point2lines.insert(*point, Vec::new());
                          }
                          point2lines.get_mut(point).unwrap().push(line);
                         });
                });

    let valid_points = point2lines.into_iter()
                                       .filter(|(_key,value)| value.len() >= 2)
                                       .fold(0,|acc,_x| acc+1);

    println!("At {:?} many points do at least two lines overlap ..", valid_points);

    let lines: Vec<Line> = read_input(input).unwrap();
    let mut point2lines: HashMap<Point, Vec<&Line>> = HashMap::new();
    lines.iter().for_each(|line| {
        let points = line.points();
        points.iter()
            .for_each(|point| {
                if !point2lines.contains_key(point) {
                    // https://users.rust-lang.org/t/creates-a-temporary-which-is-freed-while-still-in-use-again/29211
                    point2lines.insert(*point, Vec::new());
                }
                point2lines.get_mut(point).unwrap().push(line);
            });
    });

    let vp: Vec<Point> = point2lines.into_iter()
                                                         .filter(|(_key,value)| value.len() >= 2)
                                                         .map(|(key,_value)| key)
                                                         .collect();

    println!("At {:?} many points do at least two lines overlap ..", vp.len());

    Ok(())
}

fn read_input(filename: &String) ->  io::Result<Vec<Line>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let  lines =  file_reader.lines()
                                                           .map(|x|line_to_line(&x.unwrap()))
                                                           .collect();
    return Ok(lines)
}

fn line_to_line(line: &String) -> Line {
    let mut split = line.split(" -> ");
    return Line {
       start: line_to_point(split.next().unwrap()),
       end:  line_to_point(split.next().unwrap())
    }
}

fn line_to_point(line: &str) -> Point {
    let mut split = line.split(",");
    return Point {
        x: split.next().unwrap().parse::<i32>().unwrap(),
        y: split.next().unwrap().parse::<i32>().unwrap()
    }
}

fn straigt_endpoints_to_interval(a: i32, b: i32) -> Range<i32> {
    let begin = min(a,b);
    let end = max(a, b) + 1;
    begin..end
}
