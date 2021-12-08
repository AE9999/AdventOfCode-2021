use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

struct InputLine {
    input: Vec<String>, // See if we can't put this into [String; 4]
    output: Vec<String>, // See if we can't put this into [String; 4]
}

impl InputLine {
    fn decode(&self)  {
        let mut solution:  HashMap<char, char> = HashMap::new();

        let one:HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 2).unwrap().chars());
        let seven:HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 3).unwrap().chars());

        let a_value =  seven.difference(&one).next().unwrap();
        solution.insert(*a_value, 'a');
        println!("{:?} -> a", *a_value);
        //println!("{:?}", solution);

        let eight: HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 7).unwrap().chars());
        let missing_one_values: Vec<HashSet<char>> = self.input.iter()
                                                         .find(|x|x.len() == 6)
                                                         .iter()
                                                         .map(|x|HashSet::from_iter(x.chars()))
                                                         .collect();
        let d_c_e: HashSet<char> = HashSet::from_iter(missing_one_values.iter()
                                                      .map(|x| *(eight.difference(x).next().unwrap() ) ));
        let one_chars: Vec<char> = one.iter().map(|x|x.clone()).collect();
        if !one_chars.len() == 2 { panic!("Wrong assumption") }
        if d_c_e.contains(&one_chars[0]) {
            solution.insert(one_chars[0], 'c');
            solution.insert(one_chars[1], 'f');
            println!("{:?} -> c", one_chars[0]);
            println!("{:?} -> f", one_chars[1]);
        } else {
            solution.insert(one_chars[1], 'c');
            solution.insert(one_chars[0], 'f');
            println!("{:?} -> c", one_chars[1]);
            println!("{:?} -> f", one_chars[0]);
        }
        //println!("{:?}", solution);

        let four:HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 4).unwrap().chars());
        let mut four_and_seven:HashSet<char>  = seven.to_owned();
        four_and_seven.extend(four);
        if !four_and_seven.len() == 6 { panic!("Wrong assumption") }
        let nine: Vec<String> = self.input.iter()
                                            .filter(|x| {
                                                let chars = HashSet::from_iter(x.chars());
                                                let difference: HashSet<char> = chars.difference(&four_and_seven).map(|x| x.clone()).collect();
                                                x.len() == four_and_seven.len() + 1 && difference.len() == 1
                                            } ).map(|x| x.clone()).collect();
        if !nine.len() == 1 { panic!("Wrong assumption") }
        let nine_chars = HashSet::from_iter(nine[0].chars());
        let g_set : HashSet<char> = nine_chars.difference(&four_and_seven).map(|x|x.clone()).collect();
        let g_char = *(g_set.iter().next().unwrap());
        println!("{:?} -> g", g_char);
        solution.insert(g_char, 'g'); // Goed till here

        let current_values: HashSet<char> = HashSet::from_iter(solution.iter().map(|(_k,v)|v.clone()));
        let current_keys: HashSet<char> = HashSet::from_iter(solution.iter().map(|(k,_v)|k.clone()));
        let three: Vec<char> = self.input.iter().filter(|x| {
            let set : HashSet<char> = HashSet::from_iter(x.chars().map(|x|x.clone()));
            let intersection: HashSet<char>  = set.intersection(&current_values).map(|x|x.clone()).collect();
            let diff: HashSet<char> = set.difference(&current_values).map(|x|x.clone()).collect();
            intersection.len() == current_values.len() && diff.len() == 1
        }).map(|x|{
            let set : HashSet<char> = HashSet::from_iter(x.chars().map(|c|c.clone()));
            let diff : HashSet<char> = set.difference(&current_keys).map(|c|c.clone()).collect();
            diff.iter().next().unwrap().clone()
        }) .collect();
        if three.len() != 1 { panic!("Wrong assumption") }
        let d_char = *three.iter().next().unwrap();
        println!("{:?} -> d", d_char);
        solution.insert(d_char, 'd');


        println!("I'm DECOOODING {:?}", solution.len())
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input_lines = read_input(input).unwrap();
    let answer = input_lines.iter().map(|line|
        line.output.iter().map(|x|encodes1_4_7_or_8(x)).fold(0, |acc, x| acc +x)
    ).fold(0, |acc, x| acc +x);
    println!("{:?} Is the amount of times digits 1, 4, 7, or 8 appear in the output", answer);
    input_lines.iter().for_each(|x|x.decode());
    Ok(())
}

fn read_input(filename: &String) -> io::Result<Vec<InputLine>>  {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok)
        .map(|line| line2command(&line))
        .collect())
}

fn line2command(line: &String) -> InputLine {
    let mut split = line.split('|');
    InputLine {
        input: split.next().unwrap().split_whitespace().map(|x|String::from(x)).collect() ,
        output: split.next().unwrap().split_whitespace().map(|x|String::from(x)).collect()
    }
}

fn encodes1_4_7_or_8(input: &String) -> i32 {
    if ([2,3,4,7]).contains(&(input.len() as i32)) { 1 } else { 0 }
}
