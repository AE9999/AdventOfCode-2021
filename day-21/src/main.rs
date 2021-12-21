use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use regex::Regex;

struct Problem {
    starting_position_1: u32,
    starting_position_2: u32
}

impl Problem {
    fn new(starting_position_1: u32, starting_position_2: u32) -> Problem {
        Problem {
            starting_position_1,
            starting_position_2
        }
    }

    fn normalize(result: u32) -> u32 {
        if result > 10 {
            (result % 10)
        } else {
            result
        }
    }

    fn play_1(&self) -> u64  {
        let mut scores: [u64;2] = [0, 0];
        let mut die_offset: u64 = 0;
        let mut positions: [u32;2] = [self.starting_position_1, self.starting_position_2];
        let mut die_roles: u64 = 0;
        loop {

            let values_1 =  (0..3).map(|i| 1 + ((die_offset + i) % 100) ).fold(0, |acc,x| acc +x) as u32;
            die_roles += 1;
            die_offset += 3;
            positions[0] = Problem::normalize((positions[0] + (values_1 % 10)));
            scores[0] += (positions[0] as u64);
            println!("Player 1 roles {:?} and moves to space {:?} for a total score of {:?}.", values_1, positions[0], scores[0]);
            if scores[0] >= 1000 {
                return scores[1] *  die_roles
            }

            let values_2 =  (0..3).map(|i| 1 + ((die_offset + i) % 100) ).fold(0, |acc,x| acc +x) as u32;
            die_roles += 1;
            die_offset += 3;
            positions[1] = Problem::normalize(positions[1] + (values_2 % 10));
            scores[1] += (positions[1]  as u64);
            println!("Player 2 roles {:?} moves to space {:?} for a total score of {:?}.", values_2, positions[1], scores[1]);
            if scores[1] >= 1000 {
                return scores[0] *  die_roles
            }

            // if die_roles > 8 { return 0 }
        }
    }
}


fn main() -> io::Result<()> {
    let problem = read_lines(&env::args().collect::<Vec<String>>()[1]).unwrap();
    let answer = problem.play_1();
    println!("{:?} do you get if you multiply the score of the losing player by the number of times the die was rolled during the game?",
             answer);
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let mut lines = BufReader::new(File::open(filename)?).lines();
    let re = Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();

    let line = lines.next().unwrap().unwrap();
    let cap = re.captures(line.as_ref()).unwrap();
    let starting_position_1 = u32::from_str_radix(&cap[2], 10).unwrap();

    let line = lines.next().unwrap().unwrap();
    let cap = re.captures(line.as_ref()).unwrap();
    let starting_position_2 = u32::from_str_radix(&cap[2], 10).unwrap();

    Ok(Problem::new(starting_position_1, starting_position_2))
}
