use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let commands = file_to_commands(input).unwrap();
    let gamma_rate_str = (0..commands[0].len()).into_iter()
                                              .map(|x|most_common_bit(x, &commands))
                                              .collect::<String>() ;
    let epsilon_rate_str = gamma_rate_str.chars()
                                                .map(|x| char::from_digit(1-x.to_digit(10).unwrap(), 10).unwrap())
                                                .collect::<String>();
    let gamma = isize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_rate_str, 2).unwrap();
    println!("{:?} is the power consumption of the submarine ..", gamma * epsilon);

    let oxygen_generator_rating = sort_commands('1', true, &mut commands.to_vec());

    let co2_scrubber_rating  = sort_commands('0', false,  &mut commands.to_vec());
    println!("{:?} is the life support rating of the submarine..", oxygen_generator_rating * co2_scrubber_rating);
    Ok(())
}

fn file_to_commands(filename: &String) ->  io::Result<Vec<Vec<char>>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let result = file_reader.lines()
                            .filter_map(io::Result::ok)
                            .map(|line| line.as_str().chars().collect())
                            .collect();
    Ok(result)
}

fn most_common_bit(pos: usize, commands: &Vec<Vec<char>>) -> char {
    let sum = commands.iter().filter_map(|command|command[pos].to_digit(10)).fold(0, |acc,x| acc+x);
    return if sum > (commands.len() as u32) / 2 { '1' } else { '0' };
}

fn sort_commands(default: char,
                 use_most_common: bool,
                 commands: &mut Vec<Vec<char>>) -> isize {
    let mut pos = 0;
    while commands.len() > 1 {
        let most_common_bit = most_common_bit_eq(pos, default, use_most_common, commands);
        commands.retain(|x|x[pos] == most_common_bit);
        pos += 1;
    }
    return isize::from_str_radix(commands.first().unwrap().into_iter().collect::<String>().as_str(), 2).unwrap()
}

fn most_common_bit_eq(pos: usize,
                      default: char,
                      use_most_common: bool,
                      commands: &Vec<Vec<char>>) -> char {
    let sum = commands.iter().filter_map(|command|command[pos].to_digit(10)).fold(0, |acc,x| acc+x);
    let len = commands.len() as u32;
    if len % 2 == 0 && sum == len / 2 { return default };
    if use_most_common { return if sum > len / 2 { '1' } else { '0' }; }
    return if sum > len / 2 { '0' } else { '1' }
}

