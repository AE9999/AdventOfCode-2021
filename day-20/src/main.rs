use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use std::iter::Map;
use std::slice::Iter;


struct Problem {
    image_enhancement_algorithm: Vec<char>,
    image: Vec<Vec<char>>,
    default_char: char,
}

impl Problem {

    fn width(&self) -> i32 { self.image[0].len() as i32 }

    fn height(&self) -> i32 {  self.image.len() as i32 }

    fn new(image_enhancement_algorithm: Vec<char>,
           image: Vec<Vec<char>>) -> Problem {
        Problem {
            image_enhancement_algorithm,
            image,
            default_char: '.'
        }
    }

    fn print(&self) {
        println!("{:?}", vec!['!'; self.width() as usize]);
        self.image.iter().for_each(|x| {
            println!("{:?}", x.iter().collect::<String>());
        });
        println!("{:?}", vec!['!'; self.width() as usize]);
    }

    fn offboard(&self, point: &Point) -> bool {
        point.x < 0 ||
        point.x as usize >= self.image[0].len() ||
        point.y < 0 ||
        point.y as usize >= self.image.len()
    }

    fn pixel(&self, point: &Point) -> char {
        if self.offboard(point) { self.default_char } else { self.image[point.y as usize][point.x as usize] }
    }

    fn step(&mut self) {
        let mut next_image : Vec<Vec<char>> = Vec::new();
        for y in -2..self.height()+ 2 {
            next_image.push(Vec::new());
            for x in -2..self.width() + 2 {
                let value = Point::neighbours(Point {x, y}).iter()
                                                                                      .map(|point|self.pixel(point))
                                                                                      .map(|c| if c == '.' { '0' } else { '1' })
                                                                                      .collect::<String>();
                let index = usize::from_str_radix(value.as_str(), 2).unwrap();
                next_image.last_mut().unwrap().push(self.image_enhancement_algorithm[index]);
            }
        };
        self.image = next_image;

        let value = (0..9).map(|_|self.default_char).map(|c| if c == '.' { '0' } else { '1' })
                                        .collect::<String>();
        let index = usize::from_str_radix(value.as_str(), 2).unwrap();
        self.default_char = self.image_enhancement_algorithm[index];
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {

    fn step(point: &Point, dxdy: (i32, i32)) -> Point { Point { x: point.x + dxdy.0, y: point.y + dxdy.1} }

    // AE: Make this into an iterator
    fn neighbours(point: Point) -> Vec<Point> {
         [(-1,-1), (0,-1), (1,-1),
          (-1,0), (0,0), (1,0),
          (-1,1), (0,1), (1,1)].iter().map(|dxdy|Point::step( &point, *dxdy)).collect()
    }
}

// 5772 too high
fn main() -> io::Result<()> {
    let mut problem = read_lines(&env::args().collect::<Vec<String>>()[1]).unwrap();
    (0..2).for_each(|_|problem.step());
    let answer = problem.image.iter().map(|line| {
                        line.iter().filter(|c| **c == '#').collect::<String>().len()
                      }).fold(0, |acc,x| acc+x);
    println!("{:?} many pixels are lit in the resulting image", answer);
    (0..48).for_each(|_|problem.step());
    let answer = problem.image.iter().map(|line| {
        line.iter().filter(|c| **c == '#').collect::<String>().len()
    }).fold(0, |acc,x| acc+x);
    println!("{:?} many pixels are lit in the resulting image", answer);
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let input: Vec<Vec<char>> = BufReader::new(File::open(filename)?).lines()
                                                                .map(|line|  line.unwrap())
                                                                .filter(|x|!x.is_empty())
                                                                .map(|x|x.chars().collect::<Vec<char>>())
                                                                .collect();

    Ok(Problem::new(input[0].to_owned(), input[1..].to_owned()))
}
