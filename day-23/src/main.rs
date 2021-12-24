use std::cmp::Ordering;
use std::borrow::BorrowMut;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use lazy_static::lazy_static; // 1.3.0
use std::collections::BinaryHeap;

lazy_static! {
    static ref C_2_ENERGY:HashMap<char, u64> = {
        let mut map: HashMap<char, u64> = HashMap::new();
        map.insert('A', 1);
        map.insert('B', 10);
        map.insert('C', 100);
        map.insert('D', 1000);
        map
    };
}

// lazy_static! {
//     static ref ROOMS: HashMap<char, [Point;2]> = {
//         let mut map: HashMap<char, [Point;2]> = HashMap::new();
//         map.insert('A', [Point::new(3,2), Point::new(3,3)]);
//         map.insert('B', [Point::new(5,2), Point::new(5,3)]);
//         map.insert('C', [Point::new(7,2), Point::new(7,3)]);
//         map.insert('D', [Point::new(9,2), Point::new(9,3)]);
//         map
//     };
// }

lazy_static! {
    static ref ROOMS: HashMap<char, [Point;4]> = {
        let mut map: HashMap<char, [Point;4]> = HashMap::new();
        map.insert('A', [Point::new(3,2), Point::new(3,3), Point::new(3,4), Point::new(3,5)]);
        map.insert('B', [Point::new(5,2), Point::new(5,3), Point::new(5,4), Point::new(5,5)]);
        map.insert('C', [Point::new(7,2), Point::new(7,3), Point::new(7,4), Point::new(7,5)]);
        map.insert('D', [Point::new(9,2), Point::new(9,3), Point::new(9,4), Point::new(9,5)]);
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

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    problem: Vec<Vec<char>>,
    energy_cost: u64
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.energy_cost.cmp(&self.energy_cost)
            .then_with(|| self.hashable_value().cmp(&other.hashable_value()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Clone, Copy, Debug)]
struct Move {
    origin: Point,
    destination: Point,
    cost: u64
}

impl Move {
    fn new(origin: &Point, destination: &Point, cost: u64) -> Move {
        Move {
            origin: origin.clone(),
            destination: destination.clone(),
            cost
        }
    }
}

impl State {

    fn new(problem: Vec<Vec<char>>, energy_cost: u64) -> State {
        State {
            problem,
            energy_cost
        }
    }

    fn hashable_value(&self) -> String {
        let mut value: String = String::new();
        self.problem.iter().for_each(|vec|
            vec.iter().for_each(|c| value.push(*c))
        );
        value
    }

    fn height(&self) -> i32 { self.problem.len() as i32 }

    fn width(&self) -> i32  { self.problem[0].len() as i32 }

    fn value(&self, point: &Point) -> char {
        self.problem[point.y as usize][point.x as usize]
    }

    fn is_wall(&self, point: &Point) -> bool {
        self.is_on_board(point) && self.value(point) == '#'
    }

    fn is_empty(&self, point: &Point) -> bool {
        self.is_on_board(point) && self.value(point) == '.'
    }

    fn is_amphipods(&self, point: &Point) -> bool {
        self.is_on_board(point) && ['A', 'B', 'C', 'D'].contains(&self.value(point))
    }

    fn invalid_room_entry(&self, origin: &Point, destination: &Point) -> bool {
        if !self.is_amphipods(origin) { panic!("Idiot can't program") }
        if !self.is_room(destination) { panic!("Idiot can't program") }
        if !self.is_empty(destination) { panic!("Idiot can't program") }
        let c = self.value(origin);

        let start = ROOMS.iter().find(|(_k,v)| v.contains(origin) && v.contains(destination));
        if start.is_some() { return false; }

        let rooms = ROOMS.get(&c).unwrap();

        (!rooms.contains(destination))
        || (0..rooms.len()).find(|i| (!self.is_empty(&rooms[*i]) && self.value(&rooms[*i]) != c)).is_some()
    }

    fn is_immediately_outside_of_room(&self, point: &Point) -> bool {
        OUT_SIDE_OF_ROOMS.contains(point)
    }

    fn is_room(&self, point: &Point) -> bool {
        point.y != 1
    }

    fn is_done(&self) -> bool {
        ROOMS.iter()
             .find(|(k,v)| self.value(&v[0]) != **k || self.value(&v[1]) != **k )
             .is_none() // Cannot find a counter example.
    }


    fn is_on_board(&self, point: &Point) -> bool {
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
        seen.insert(*origin);

        let started_in_room = self.is_room(origin);
        let mut deque: VecDeque<(Point, u64)> = VecDeque::new();


        origin.neighbours().iter().for_each(|destination|
            deque.push_back((destination.clone(), 1))
        );

        while !deque.is_empty() {
            let (destination, steps) = deque.pop_front().unwrap();
            if !seen.contains(&destination) {
                seen.insert(destination);

                if self.is_wall(&destination) // can't walk through walls
                   || self.is_amphipods(&destination) // can't walk through amphods
                   || (self.is_room(&destination) && self.invalid_room_entry(&origin, &destination))
                {
                    continue
                }

                let can_stop = !(!started_in_room && !self.is_room(&destination) // We started in the hallway and end in the hallway
                                       || (started_in_room && self.is_immediately_outside_of_room(&destination))); // Amphipods will never stop on the space immediately outside any room.// We can't make a move of size 0

                if can_stop {
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

    fn update_value(&mut self, point: &Point, value: char) {
        self.problem[point.y as usize][point.x as usize] = value
    }

    fn perform_move(&mut self, possible_move: Move) {
        if !self.is_empty(&possible_move.destination) || !self.is_amphipods(&possible_move.origin) { panic!("Idiot programmer") }
        self.update_value(&possible_move.destination, self.value(&possible_move.origin));
        self.update_value(&possible_move.origin, '.');
        self.energy_cost += possible_move.cost;
    }

    fn possible_moves(&self) -> Vec<Move> {
        let mut rvalue = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let point = Point::new(x, y);
                if self.is_amphipods(&point) {
                    let mut res = self.possible_moves_for_amphipods(&point);
                    rvalue.append(res.as_mut());
                }
            }
        }
        rvalue
    }

    fn print(&self) {
        println!("{:?}", String::from_iter(vec!['!'; self.width() as usize]));
        self.problem.iter().for_each(|x| {
            println!("{:?}", x.iter().collect::<String>());
        });
        println!("{:?}", String::from_iter(vec!['!'; self.width() as usize]));
    }

}

impl Problem {

    fn new(problem: Vec<Vec<char>>) -> Problem {
        Problem {
            problem,
        }
    }

    // Yeah fuck it why not use Dijkstra?

    // How do we deal with dead branches?
    fn solve1(&self) -> u64 {

        let mut heap : BinaryHeap<State> = BinaryHeap::new();
        let mut dist: HashMap<Vec<Vec<char>>, u64> = HashMap::new();
        let mut terminated_states: HashSet<Vec<Vec<char>>> = HashSet::new();

        heap.push(State::new(self.problem.clone(), 0) );
        while let state = heap.pop().unwrap() {
            // state.print();
            if state.is_done() {
                return state.energy_cost
            }
            if state.energy_cost > *(dist.get(&state.problem).unwrap_or(&u64::MAX)) { continue; }

            let mut contained_possible_move = false;

            for possible_move in state.possible_moves() {
                let mut next_state = state.clone();
                next_state.perform_move(possible_move);

                if terminated_states.contains(&next_state.problem) { continue }

                if next_state.energy_cost < *(dist.get(&next_state.problem).unwrap_or(&u64::MAX)) {
                    contained_possible_move = true;
                    dist.insert(next_state.problem.clone(), next_state.energy_cost.clone());
                    heap.push(next_state);
                }
            }

            if !contained_possible_move {
                terminated_states.insert(state.problem);
            }
        }
        panic!("End state should be reachable");
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
    println!("{:?} is the least energy required to organize the amphipods", problem.solve1());
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let mut problem: Vec<Vec<char>> = Vec::new();
    BufReader::new(File::open(filename)?).lines().for_each(|line|{
        problem.push(line.unwrap().chars().collect::<Vec<char>>())
    });
    Ok(Problem::new(problem))
}
