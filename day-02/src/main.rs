use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

struct Command {
    command: String,
    amount: i64
}

struct Postion {
    x: i64,
    y: i64,
}

struct AdvancedPosition {
    x: i64,
    y: i64,
    aim: i64
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let commands = file_to_commands(input).unwrap();

    let mut postion = Postion{ x: 0,  y: 0  };
    commands.iter().for_each(|command| update_position(&mut postion, &command));
    println!("{:?} is what you get if you multiply your final horizontal position by your final depth? ..",
             postion.x * postion.y);

    let mut advanced_position = AdvancedPosition{ x: 0, y:0, aim: 0  };
    commands.iter()
            .for_each(|command| update_advanced_position(&mut advanced_position,
                                                                    &command));
    println!("{:?} is what you get if you multiply your final horizontal position by your final depth? ..",
             advanced_position.x * advanced_position.y);
    Ok(())
}

fn file_to_commands(filename: &String) -> io::Result<Vec<Command>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok)
        .map(|line| line2command(&line) )
        .collect())
}

fn line2command(line: &String) -> Command {
    let mut split = line.split(" ");
    return Command {
        command: split.next().unwrap().parse().unwrap(),
        amount: split.next().unwrap().parse::<i64>().unwrap()
    }
}

fn update_position(postion: &mut Postion, command: &Command) {
    match command.command.as_ref() {
        "forward" => postion.x += command.amount,
        "down" => postion.y += command.amount,
        "up" => postion.y -= command.amount,
        _ => panic!("Unexpected input")
    }
}

fn update_advanced_position(advanced_postion: &mut AdvancedPosition, command: &Command) {
    match command.command.as_ref() {
        "down" => advanced_postion.aim += command.amount,
        "up" => advanced_postion.aim -= command.amount,
        "forward" => {
            advanced_postion.x += command.amount;
            advanced_postion.y += advanced_postion.aim * command.amount
        },
        _ => panic!("Unexpected input")
    }
}

