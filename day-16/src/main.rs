use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let lines = read_lines(input).unwrap();
    lines.iter().for_each(|x|{
        let decoded = decode(x);
        let version = usize::from_str_radix(decoded[0..3].as_ref(), 2).unwrap();
        let type_id = usize::from_str_radix(decoded[3..6].as_ref(), 2).unwrap();
        if version == 4 { decode_literal_value(decoded[6..].as_ref()) }
    });

    println!("{:?} do you get if you add up the version numbers in all packets?", 0);

    Ok(())
}

fn decode_literal_value(decoded: &str) -> usize {
    let result: String = String::new();
    let mut offset = 0 ;
    loop {
        if decoded[offset..offset + 1] == '1' { break }
        usize::from_str_radix(decoded[offset+1..offset+5].as_ref(), 2).unwrap();
        offset += 5
    }
    usize::from_str_radix(result.as_str(), 10).unwrap()
}

fn parse_lenght(decoded: &str) -> Vec<usize> {
    let r : Vec<usize> = Vec::new();
    r
}

fn parse_sub_packets(decoded: &str) -> (Vec<usize>, usize) {
    let r : Vec<usize> = Vec::new();
    (r, 0)
}

fn parse_operator_packet(decoded: &str) {
    let length_type_id = decoded[0];
    if length_type_id == '0' {
        let total_length_in_bits = usize::from_str_radix(decoded[1..16].as_ref(), 2).unwrap();


    } else {
        let number_of_sub_packets = usize::from_str_radix(decoded[1..12].as_ref(), 2).unwrap();
    }
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

fn read_lines(filename: &String) -> io::Result<Vec<String>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

