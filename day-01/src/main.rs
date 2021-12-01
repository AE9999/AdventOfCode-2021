use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let values = file_to_vec(input)?;
    let initial = (0, values[0]);
    let result = values.iter()
                                .fold(initial,
                                      |amount_higher_and_previous_value,x| evaluate(amount_higher_and_previous_value, x));
    println!("{:?} measurements are larger than the previous measurement ..", result.0);

    let initial_window: (i32, Vec<i32>) = (0, values[0..3].to_vec());
    let result_window = values.windows(3).fold(initial_window,
                                               |amount_higher_and_previous_value, x|
                                                                evaluate_window(amount_higher_and_previous_value, x.to_vec()));
    println!("{:?}  sums are larger than the previous sum ..", result_window.0);
    Ok(())
}

fn evaluate(amount_higher_and_previous_value: (i32, i32), x: &i32) -> (i32, i32) {
    let amount_higher = if x > &(amount_higher_and_previous_value.1) {
                          amount_higher_and_previous_value.0 + 1
                        } else {
                          amount_higher_and_previous_value.0
                        };
    return (amount_higher, *x);
}

fn evaluate_window<'a>(amount_higher_and_previous_value: (i32, Vec<i32>), x: Vec<i32>) -> (i32, Vec<i32>) {
    let amount_previous = (amount_higher_and_previous_value.1).iter().fold(0, |acc,x| acc + x);
    let amount_current  = (x).iter().fold(0, |acc,x| acc + x);
    return (if amount_current > amount_previous { amount_higher_and_previous_value.0  + 1}
            else {amount_higher_and_previous_value.0},
           x)
}

fn file_to_vec(filename: &String) -> io::Result<Vec<i32>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok)
                          .map(|x| x.parse::<i32>().unwrap())
                          .collect())
}
