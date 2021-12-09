use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

struct InputLine {
    input: Vec<String>, // See if we can't put this into [String; 4]
    output: Vec<String>, // See if we can't put this into [String; 4]
}

impl InputLine {

    fn recalculate(&self) -> HashMap<&str, char> {
        let mut rvalue : HashMap<&str, char> = HashMap::new();
        rvalue.insert("abcefg", '0');
        rvalue.insert("cf" , '1');
        rvalue.insert("acdeg", '2');
        rvalue.insert("acdfg" , '3');
        rvalue.insert("bcdf", '4');
        rvalue.insert("abdfg", '5');
        rvalue.insert("abdefg", '6');
        rvalue.insert("acf", '7');
        rvalue.insert("abcdefg", '8');
        rvalue.insert("abcdfg", '9');
        return rvalue;
    }

    fn decode(&self) -> i32 {
        let mut solution:  HashMap<char, char> = HashMap::new();

        let one:HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 2).unwrap().chars());
        let seven:HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 3).unwrap().chars());

        let d: Vec<char> = seven.difference(&one).map(|x|x.clone()).collect();
        if d.len() != 1 { panic!("Wrong assumption") }
        let a_value =  d[0];
        solution.insert(a_value, 'a');

        let eight: HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 7).unwrap().chars());
        let missing_one_values: Vec<HashSet<char>> = self.input.iter()
                                                         .filter(|x|x.len() == 6)
                                                         .map(|x|HashSet::from_iter(x.chars()))
                                                         .collect();
        let d_c_e: HashSet<char> = HashSet::from_iter(missing_one_values.iter()
                                                      .map(|x| *(eight.difference(x).next().unwrap() ) ));
        let one_chars: Vec<char> = one.iter().map(|x|x.clone()).collect();
        if one_chars.len() != 2 { panic!("Wrong assumption") } // This seems to be random and prone to error ..
        if d_c_e.contains(&one_chars[0]) {
            solution.insert(one_chars[0], 'c');
            solution.insert(one_chars[1], 'f');
        } else {
            solution.insert(one_chars[1], 'c');
            solution.insert(one_chars[0], 'f');
        }

        let four:HashSet<char> = HashSet::from_iter( self.input.iter().find(|x|x.len() == 4).unwrap().chars());
        let mut four_and_seven:HashSet<char>  = seven.to_owned();
        four_and_seven.extend(four);
        if four_and_seven.len() != 5 { panic!("Wrong assumption") }
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
        solution.insert(g_char, 'g'); // Goed till here

        let current_values: HashSet<char> = HashSet::from_iter(solution.iter().map(|(k,_v)|k.clone()));
        let three: Vec<char> = self.input.iter().filter(|x| x.len() == 5)
                                                .filter(|x| {
            let set : HashSet<char> = HashSet::from_iter(x.chars().map(|x|x.clone()));
            let intersection: HashSet<char>  = set.intersection(&current_values).map(|x|x.clone()).collect();
            let diff: HashSet<char> = set.difference(&current_values).map(|x|x.clone()).collect();
            intersection.len() == current_values.len() && diff.len() == 1
        }).map(|x|{
            let set : HashSet<char> = HashSet::from_iter(x.chars().map(|c|c.clone()));
            let diff : HashSet<char> = set.difference(&current_values).map(|c|c.clone()).collect();
            diff.iter().next().unwrap().clone()
        }) .collect();
        if three.len() != 1 { panic!("Wrong assumption") }
        let d_char = *three.iter().next().unwrap();
        solution.insert(d_char, 'd');


        let current_values: HashSet<char> = HashSet::from_iter(solution.iter().map(|(k,_v)|k.clone()));
        let nine: Vec<char>  = self.input.iter().filter(|x|x.len() == 6)
            .filter(|x| {
                let set : HashSet<char> = HashSet::from_iter(x.chars().map(|x|x.clone()));
                let intersection: HashSet<char>  = set.intersection(&current_values).map(|x|x.clone()).collect();
                let diff: HashSet<char> = set.difference(&current_values).map(|x|x.clone()).collect();
                intersection.len() == current_values.len() && diff.len() == 1
            }).map(|x|{
            let set : HashSet<char> = HashSet::from_iter(x.chars().map(|c|c.clone()));
            let diff : HashSet<char> = set.difference(&current_values).map(|c|c.clone()).collect();
            diff.iter().next().unwrap().clone()
        }) .collect();
        if nine.len() != 1 { panic!("Wrong assumption") }
        let b_char = *nine.iter().next().unwrap();
        solution.insert(b_char, 'b');

        let current_values: HashSet<char> = HashSet::from_iter(solution.iter().map(|(k,_v)|k.clone()));
        let eight: Vec<char>  = self.input.iter().filter(|x|x.len() == 7).map(|x|{
            let set : HashSet<char> = HashSet::from_iter(x.chars().map(|c|c.clone()));
            let diff : HashSet<char> = set.difference(&current_values).map(|c|c.clone()).collect();
            diff.iter().next().unwrap().clone()
        }) .collect();
        if eight.len() != 1 { panic!("Wrong assumption") }
        let e_char = *eight.iter().next().unwrap();
        solution.insert(e_char, 'e');

        let parsed: String = self.output.iter().map(|x| {
            let mut key : Vec<char> = x.chars().map(|key|*solution.get(&key).unwrap()).collect();
            key.sort();
            let key: String = key.into_iter().collect();
            self.recalculate().get(key.as_str()).unwrap().clone()
        }).collect();

        parsed.parse::<i32>().unwrap()
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
    let answer = input_lines.iter().fold(0,  |acc, x|acc + x.decode());
    println!("{:} is the answer you get if you add up all of the output values", answer);
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
