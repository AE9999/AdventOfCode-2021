use std::collections::HashMap;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use regex::Regex;

type PathLenght = u32;

type PathAmount = u32;

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
            die_roles += 3;
            die_offset += 3;
            positions[0] = Problem::normalize((positions[0] + (values_1 % 10)));
            scores[0] += (positions[0] as u64);
            if scores[0] >= 1000 {
                return scores[1] *  die_roles
            }

            let values_2 =  (0..3).map(|i| 1 + ((die_offset + i) % 100) ).fold(0, |acc,x| acc +x) as u32;
            die_roles += 3;
            die_offset += 3;
            positions[1] = Problem::normalize(positions[1] + (values_2 % 10));
            scores[1] += (positions[1]  as u64);
            if scores[1] >= 1000 {
                return scores[0] *  die_roles
            }
        }
    }

    fn calculate_endpoints(&self, position: u32, current_steps: u32, current_score: u64) -> HashMap<PathLenght,
                                                                                                    PathAmount> {
        let mut rvalue: HashMap<u32, u32> =  HashMap::new();
        let current_steps_ = current_steps + 1;

        // take one,
        let position_ = Problem::normalize((position + 1));
        let current_score_ =  current_score + (position_  as u64);

        if current_score_ >= 21 {
           *rvalue.entry(current_steps_).or_insert(0) += 1;
        } else {
            let sub_problem = self.calculate_endpoints(position_, current_steps_, current_score_);
            sub_problem.iter().for_each(|(k,v)|
                *rvalue.entry( *k).or_insert(0) += v
            )
        }

        // take two
        let position_ = Problem::normalize((position + 2));
        let current_score_ =  current_score + (position_  as u64);
        if current_score_ >= 21 {
            *rvalue.entry(current_steps_).or_insert(0) += 1;
        } else {
            let sub_problem = self.calculate_endpoints(position_, current_steps_, current_score_);
            sub_problem.iter().for_each(|(k,v)|
                *rvalue.entry( *k).or_insert(0) += v
            )
        }

        // take three
        let position_ = Problem::normalize((position + 3));
        let current_score_ =  current_score + (position_  as u64);
        if current_score_ >= 21 {
            *rvalue.entry(current_steps_).or_insert(0) += 1;
        } else {
            let sub_problem = self.calculate_endpoints(position_, current_steps_, current_score_);
            sub_problem.iter().for_each(|(k,v)|
                *rvalue.entry( *k).or_insert(0) += v
            )
        }
        rvalue
    }

    fn play_2(&self) -> u64 {
        // So what are all the lenghts of all the paths that lead to victory per player

        let completing_one_games = self.calculate_endpoints(self.starting_position_1, 0, 0);
        println!("{:?}", completing_one_games);
        let completing_two_games = self.calculate_endpoints(self.starting_position_2, 0, 0);
        println!("{:?}", completing_two_games);

        let winning_games_one = completing_one_games.iter().map(|(k,v)|{
            let games_beat = completing_two_games.iter()
                                                    .filter(|(k2,v2)| k <= k2)
                                                    .map(|(k2,v2)| v2)
                                                    .fold(0, |acc,x|acc+x);
            games_beat * v // The amount off posibilties is the
        }).fold(0,|acc,x| acc+x);
        println!("Winning games one: {:?}", winning_games_one);


        0
    }
}


fn main() -> io::Result<()> {
    let problem = read_lines(&env::args().collect::<Vec<String>>()[1]).unwrap();
    let answer = problem.play_1();
    println!("{:?} do you get if you multiply the score of the losing player by the number of times the die was rolled during the game?",
             answer);
    let answer = problem.play_2();

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
