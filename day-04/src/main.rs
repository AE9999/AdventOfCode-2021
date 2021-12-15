use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
struct Point {
    x: usize,
    y: usize
}

struct Board {
    value2location: HashMap<i32, Point>,
    location2value: HashMap<Point, i32>,
    lenght: usize,
    height: usize
}

impl Board {

    fn get_undrawn_values(&self, drawn_numbers: &HashSet<i32>) -> i32 {
        (0..self.lenght).map(|x|
            (0..self.height).fold(0, |acc,y| {
                let number = self.get_number(&Point {x, y});
                return if ! drawn_numbers.contains(&number) {
                    acc + number
                } else {
                    acc
                }
            })
        ).fold(0, |acc,value| acc + value)
    }

    fn get_number(&self, point: &Point) -> i32 {
        self.location2value.get(point).unwrap().clone()
    }

    fn check_board(&self, last_drawn_number: &i32, drawn_numbers: &HashSet<i32>) -> i32 {
        if self.value2location.contains_key(last_drawn_number) {
            let point = self.value2location.get(last_drawn_number).unwrap().clone();
            let x_ = point.x;
            let y_ = point.y;

            let found_value = (0..self.lenght).map(|x| Point { x, y: y_ })
                                                          .find(|point|  !drawn_numbers.contains(&self.get_number(point)));
            if found_value.is_none() {
                return last_drawn_number * self.get_undrawn_values(drawn_numbers);
            }
            let found_value = (0..self.height).map(|y| Point { x: x_, y })
                                                         .find(|point|  !drawn_numbers.contains(&self.get_number(point)));
            if found_value.is_none() {
                return last_drawn_number * self.get_undrawn_values(drawn_numbers)
            }
        }
        return -1
    }

    fn simulate_game(&self, numbers: &Vec<i32>) -> (usize, i32) {
        let mut my_numbers = numbers.clone();
        let mut drawn_numbers: HashSet<i32> = HashSet::new();
        loop {
            let last_drawn_number = my_numbers.pop().unwrap();
            drawn_numbers.insert(last_drawn_number);
            let rvalue = self.check_board(&last_drawn_number, &drawn_numbers);
            if rvalue >= 0 { return (drawn_numbers.len(), rvalue); }
        }
    }
}

struct Game {
    numbers: Vec<i32>,
    boards: Vec<Board>,
    drawn_numbers: HashSet<i32>
}

impl Game {

    fn play(&mut self) -> i32 {
        while !(self.numbers.is_empty()) {
            let last_drawn_number = self.numbers.pop().unwrap();
            self.drawn_numbers.insert(last_drawn_number);

            // Kinda fun thomas, because I mode all the methods read only this will compile.
            // We're just going to assume that we only have a single winning board
            let res = self.boards.iter().map(|board| board.check_board(&last_drawn_number, &self.drawn_numbers))
                                                   .find(|value|value >= &0);
            if res.is_some() { return res.unwrap() }
        }
        panic!("We should not reach this");
    }

    fn play_losing(&mut self) -> i32 {
        let mut results: Vec<(usize, i32)> = self.boards.iter().map(|board| board.simulate_game(&self.numbers)).collect();
        results.sort_by_key(|x| x.0);
        results.reverse();
        return results.first().unwrap().1
    }
}

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut game: Game = read_input(input).unwrap();
    println!("{:?} will be your final score be if you choose the winning board ..", game.play());

    let mut game: Game = read_input(input).unwrap(); // Simply re-reading the input is easier.
    println!("{:?} will be the score of the losing board ..", game.play_losing());

    return Ok(())
}


fn read_input(filename: &String) ->  io::Result<Game> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let mut lines =  file_reader.lines();

    let mut numbers: Vec<i32> =  lines
                                           .next()
                                           .unwrap()
                                           .unwrap()
                                           .split(",")
                                           .map(|x|x.parse::<i32>().unwrap())
                                           .collect();
    numbers.reverse();

    lines.next(); // Skip empty line

    let mut boards: Vec<Board> = Vec::new();
    let mut current_board : Vec<Vec<i32>> = Vec::new();

    lines.for_each(|line|
        if line.as_ref().unwrap().is_empty() {
            boards.push(create_board(&current_board));
            current_board = Vec::new();
        } else {
            current_board.push(line.unwrap().split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect())
        }
    );

    boards.push(create_board(&current_board));

    return Ok(Game {
        numbers: numbers,
        boards: boards,
        drawn_numbers: HashSet::new()
    })
}

fn create_board(current_board : &Vec<Vec<i32>>) -> Board {
    let mut value2location: HashMap<i32, Point> = HashMap::new();
    let mut location2value: HashMap<Point, i32> = HashMap::new();
    (0..current_board[0].len()).for_each(|x| (0..current_board.len()).for_each(|y|
            {
              value2location.insert(current_board[y][x], Point { x, y} );
              location2value.insert(Point { x, y}, current_board[y][x]);
            }
    ));

    return Board {
        value2location,
        location2value,
        lenght :current_board[0].len(),
        height: current_board.len()
    }
}
