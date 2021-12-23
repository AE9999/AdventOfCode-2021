use std::collections::HashMap;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;

struct Problem {
    problem: Vec<Vec<char>>,
    c2energy: HashMap<char, u32>,
    rooms: HashMap<char, [Point;2]>,
    outside_of_rooms: [Point;4]
}

impl Problem {

    fn new(problem: Vec<Vec<char>>) -> Problem {

        let mut c2energy: HashMap<char, u32> = HashMap::new();
        c2energy.insert('a', 1);
        c2energy.insert('b', 10);
        c2energy.insert('c', 100);
        c2energy.insert('d', 1000);

        let mut rooms: HashMap<char, [Point;2]> = HashMap::new();
        rooms.insert('a', [Point::new(3,2), Point::new(3,3)]);
        rooms.insert('b', [Point::new(5,2), Point::new(5,3)]);
        rooms.insert('c', [Point::new(7,2), Point::new(7,3)]);
        rooms.insert('d', [Point::new(9,2), Point::new(9,3)]);

        let mut outside_of_rooms = [Point::new(3,1),
                                             Point::new(5,1),
                                             Point::new(7,1),
                                             Point::new(9,1)];
        Problem {
            problem,
            c2energy,
            rooms,
            outside_of_rooms
        }
    }

    fn is_wall(&self, point: &Point) -> bool {
        self.problem[point.y as usize][point.x as usize] == '#'
    }

    fn is_empty(&self, point: &Point) -> bool {
        self.problem[point.y as usize][point.x as usize] == '.'
    }

    fn is_immediately_outside_of_room(&self, point: &Point) -> bool {
        self.outside_of_rooms.contains(point)
    }

    fn is_room(&self, point: &Point) -> bool {
        point.y != 1
    }

    fn is_my_room_and_room_is_cleard(point: &Point) {

    }

    fn on_board(&self, point: Point) -> bool {
        point.y >= 0
        && point.y < self.problem.len() as i32
        && point.x >= 0
        && point.x < self.problem[point.y as usize].len() as i32
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

    fn step(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn main()  -> io::Result<()> {
    let problem = read_lines(&env::args().collect::<Vec<String>>()[1]).unwrap();
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let mut problem: Vec<Vec<char>> = Vec::new();
    BufReader::new(File::open(filename)?).lines().for_each(|line|{
        problem.push(line.unwrap().chars().collect::<Vec<char>>())
    });
    Ok(Problem::new(problem))
}
