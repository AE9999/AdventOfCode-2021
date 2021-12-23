use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use lazy_static::lazy_static; // 1.3.0

lazy_static! {
    static ref C_2_ENERGY:HashMap<char, u32> = {
        let mut map: HashMap<char, u32> = HashMap::new();
        map.insert('a', 1);
        map.insert('b', 10);
        map.insert('c', 100);
        map.insert('d', 1000);
        map
    };
}

lazy_static! {
    static ref ROOMS: HashMap<char, [Point;2]> = {
        let mut map: HashMap<char, [Point;2]> = HashMap::new();
        map.insert('a', [Point::new(3,2), Point::new(3,3)]);
        map.insert('b', [Point::new(5,2), Point::new(5,3)]);
        map.insert('c', [Point::new(7,2), Point::new(7,3)]);
        map.insert('d', [Point::new(9,2), Point::new(9,3)]);
        map
    };
}

lazy_static! {
    static ref OUT_SIDE_OF_ROOMS: [Point;4] = [Point::new(3,1),
                                              Point::new(5,1),
                                              Point::new(7,1),
                                              Point::new(9,1)];
}

struct Problem {
    problem: Vec<Vec<char>>,
}

struct State {
    problem: Vec<Vec<char>>,
    possible_moves: Vec<Move>,
    energy_cost: u64
}

struct Move {
    origin: Point,
    destination: Point,
    cost: u32
}

impl Move {
    fn new(origin: &Point, destination: &Point, cost: u32) -> Move {
        Move {
            origin: origin.clone(),
            destination: destination.clone(),
            cost
        }
    }
}

impl State {

    fn height(&self) -> i32 { self.problem.len() as i32 }

    fn width(&self) -> i32  { self.problem[0].len() as i32 }

    fn value(&self, point: &Point) -> char {
        self.problem[point.y as usize][point.x as usize]
    }

    fn is_wall(&self, point: &Point) -> bool {
        self.on_board(point) && self.value(point) == '#'
    }

    fn is_empty(&self, point: &Point) -> bool {
        self.on_board(point) && self.value(point) == '.'
    }

    fn is_amphipods(&self, point: &Point) -> bool {
        self.on_board(point) && ['a', 'b', 'c', 'd'].contains(&self.value(point))
    }

    fn is_immediately_outside_of_room(&self, point: &Point) -> bool {
        OUT_SIDE_OF_ROOMS.contains(point)
    }

    fn is_room(&self, point: &Point) -> bool {
        point.y != 1
    }

    fn valid_move_into_room(&self, origin: &Point, destination: &Point) -> bool {
        if !self.is_room(destination) { panic!("Idiot cannot program"); };
        let c = self.value(origin);
        let rooms = ROOMS.get(&c).unwrap();
        rooms.contains(destination)
        && (self.is_empty(&rooms[0]) || self.value(destination) == c)
        && (self.is_empty(&rooms[1]) || self.value(destination) == c)
    }

    fn on_board(&self, point: &Point) -> bool {
        point.y >= 0
        && point.y < self.problem.len() as i32
        && point.x >= 0
        && point.x < self.problem[point.y as usize].len() as i32
    }

    fn possible_moves_for_amphipods(&self, origin: &Point) -> Vec<Move> {
        if !self.is_amphipods(origin) { panic!("Can't program") }

        let amphod = self.value(origin);
        let mut rvalue : Vec<Move> = Vec::new();
        let mut seen: HashSet<Point> = HashSet::new();
        let started_in_room = self.is_room(origin);
        let mut deque: VecDeque<(Point, u32)> = VecDeque::new();

        deque.push_back((*origin, 0));
        while !deque.is_empty() {
            let (destination, steps) = deque.pop_front().unwrap();
            if !seen.contains(&destination) {
                seen.insert(destination);

                if self.is_wall(&destination) // can't walk through walls
                   || self.is_amphipods(&destination) // can't walk through amphods
                   || (self.is_room(&destination) && !ROOMS.get(&amphod).unwrap().contains(&destination)) // wrong room
                   || (!started_in_room && self.valid_move_into_room(origin, &destination))  // Amphipods will never move from the hallway into a room unless that room is their destination room and that room contains no amphipods which do not also have that room as their own destination.
                {
                    continue
                }

                if !( (!started_in_room && !self.is_room(&destination)) // We started in the hallway and end in the hallway
                     || (started_in_room && self.is_immediately_outside_of_room(&destination)) // Amphipods will never stop on the space immediately outside any room.
                     || (destination == *origin) // We can't make a move of size 0
                ){
                    // Add as a possible endpoint
                    rvalue.push(Move::new(origin, &destination, C_2_ENERGY.get(&amphod).unwrap() *steps))
                }

                // Look for more moves
                destination.neighbours().iter().for_each(|destination|
                    deque.push_back((destination.clone(), steps+1))
                )
            }
        }
        rvalue
    }

    fn possible_moves(&self) -> Vec<Move> {
        let mut rvalue = Vec::new();
       rvalue
    }

}

impl Problem {

    fn new(problem: Vec<Vec<char>>) -> Problem {
        Problem {
            problem,
        }
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

    fn step(&self, dx: i32, dy: i32) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }

    fn neighbours(&self) -> Vec<Point> {
        [(-1,0), (0,1), (1,0), (0, -1)].iter()
                                       .map(|dxdy| self.step(dxdy.0, dxdy.1))
                                       .collect()
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
