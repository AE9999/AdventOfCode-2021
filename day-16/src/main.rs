use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;


struct Node {
    version: usize,
    type_id: usize,
    value: Option<usize>,
    children: Vec<Node>,
}

impl Node {
    fn sum_version(&self) -> usize {
        self.version + self.children.iter()
                                    .map(|child| child.sum_version())
                                    .fold(0, |acc,x| acc + x)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let line = read_lines(input).unwrap();
    let (tree, _) = parse(decode(&line).as_str());
    println!("{:?} do you get if you add up the version numbers in all packets?", tree.sum_version());
    Ok(())
}

fn parse(decoded: &str) -> (Node, usize) {
    let version = usize::from_str_radix(decoded[0..3].as_ref(), 2).unwrap();
    let type_id = usize::from_str_radix(decoded[3..6].as_ref(), 2).unwrap();
    let mut value: Option<usize> = None;
    if type_id == 4 {
        let (value_, offset_) = decode_literal_value(decoded[6..].as_ref());
        value = Option::from(value_);
        let node = Node { version, type_id, value, children: Vec::new() };
        (node, 6 + offset_)
    } else {
        let length_type_id: usize =  usize::from_str_radix(decoded[6..7].as_ref(), 10).unwrap();
        let mut children: Vec<Node> = Vec::new();
        let mut parsed_bits = 0;
        if length_type_id == 0 {
            let total_length_in_bits = usize::from_str_radix(decoded[7..22].as_ref(), 2).unwrap();
            while parsed_bits < total_length_in_bits {
                let (child, offset) = parse(decoded[22+parsed_bits..].as_ref());
                children.push(child);
                parsed_bits += offset;
            }
            if parsed_bits > total_length_in_bits { panic!("Unexpected behavior") }
            (Node { version, type_id, value, children}, 22+parsed_bits)
        } else {
            let number_of_sub_packets = usize::from_str_radix(decoded[7..18].as_ref(), 2).unwrap();
            (0..number_of_sub_packets).for_each(|_x|{
                let (child, offset) = parse(decoded[18+parsed_bits..].as_ref());
                children.push(child);
                parsed_bits += offset;
            });
            (Node { version, type_id, value, children}, 18+parsed_bits)
        }
    }
}

fn decode_literal_value(decoded: &str) -> (usize, usize) {
    let mut result: String = String::new();
    let mut offset = 0 ;
    loop {
        let group_bit = usize::from_str_radix(decoded[offset..offset + 1].as_ref(), 10).unwrap();
        decoded[offset+1..offset+5].chars().for_each(|x|result.push(x));
        offset += 5;
        if  group_bit == 0 { break }
    }
    (usize::from_str_radix(result.as_str(), 2).unwrap(), offset)
}

fn decode(value: &String) ->  String {
    let mut decoded = String::from("");
    value.chars()
         .map(|c| expand(&c))
         .for_each(|s| s.chars()
                              .for_each(|c| decoded.push(c)));
    decoded
}

fn expand(c: &char) -> &'static str {
    return match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("could not decode ..")
    }
}

fn read_lines(filename: &String) -> io::Result<String> {
    let file_in = File::open(filename)?;
    let  file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().next().unwrap().unwrap())
}

