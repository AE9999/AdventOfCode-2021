use std::cmp::min;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;
use std::ops::Range;

use lazy_static::lazy_static; // 1.3.0
use regex::Regex; // 1.1.5

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+)\sx=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();
}

struct Problem {
    instructions: Vec<Instruction>
}

impl Problem {
    fn solve_1(&self) -> i32 {
        let mut turnedon = 0;
        for x in -51..51 {
            for y in -51..51 {
                for z in -51..51 {
                    let point = Point { x, y, z };
                    let mut on = false;
                    self.instructions.iter().for_each(|instruction|{
                        if instruction.contains(&point) { on = instruction.on }
                    });
                    if on {
                        turnedon += 1;
                    }
                }
            }
        }
        turnedon
    }

    fn regions2size(covered_regions: &Vec<Box>) -> u64 {
        covered_regions.iter().map(|region|region.region()).fold(0, |acc, x|acc+x)
    }

    fn check_covered_stuff(stuff: &Vec<Box>) {
        (0..stuff.len()).for_each(|i|{
            (i+1..stuff.len()).for_each( |j|{
                if stuff[i].intersects(&(stuff[j])) {
                    panic!("You can't program boy {:?} and {:?} intersect ", stuff[i], stuff[j]);
                }
            })
        });
    }

    fn solve_2(&self) -> u64 {
        let mut covered_regions: Vec<Box> = Vec::new();

        self.instructions.iter().for_each(|instruction| {

            let mut covered_regions_: Vec<Box> = Vec::new();

            if instruction.on {
                // We are going to keep all original on areas and add unique uncovered ones
                for covered_region in &covered_regions {
                    covered_regions_.push(covered_region.clone());
                }
                Problem::check_covered_stuff(&covered_regions_);

                // Calculate all oncovered areas and add them
                let mut stack: Vec<Box> = Vec::new();
                stack.push(instruction.to_box());
                while !(stack.is_empty()) {
                    let candidate_box = stack.pop().unwrap();
                    let mut subsumed = false;
                    for covered_region in &covered_regions {
                        if !covered_region.intersects(&candidate_box) { continue }
                        let mut newly_covered_regions = candidate_box.uncovered_areas(covered_region);
                        stack.append(newly_covered_regions.as_mut());
                        subsumed = true;
                        break;
                    }
                    if !subsumed {
                        covered_regions_.push(candidate_box)
                    }
                }

            } else {
                // We are going to split all regions into unique ..
                let off_box = instruction.to_box();
                // println!("Doing a substraction of {:?} with size {:?}", off_box, off_box.region());
                for on_region in &covered_regions {
                    if !on_region.intersects(&off_box) {
                        // println!("{:?} does not intersect with {:?} just adding it", on_region, off_box);
                        covered_regions_.push(on_region.clone());
                        Problem::check_covered_stuff(&covered_regions_);
                    } else {
                        let intersection = on_region.intersection(&off_box); // Dumb coding.
                        let mut still_covered_areas = on_region.minus(&intersection);
                        /* println!("{:?} (s:{:?}) did intersect with {:?}, resulting in {:?} with size {:?}", on_region,
                                                                                    on_region.region(),
                                                                                    off_box,
                                                                                    still_covered_areas,
                                                                                    Problem::regions2size(&still_covered_areas));*/
                        covered_regions_.append(still_covered_areas.as_mut());
                        Problem::check_covered_stuff(&covered_regions_);
                    }
                }
            }
            Problem::check_covered_stuff(&covered_regions_);
            covered_regions = covered_regions_;
        });

        Problem::regions2size(&covered_regions)
    }
}

#[derive(Debug)]
struct Instruction  {
    on: bool,
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>
}

impl Instruction {
    fn contains(&self, point: &Point) -> bool {
        self.x_range.contains(&point.x)
        && self.y_range.contains((&point.y))
        && self.z_range.contains((&point.z))
    }
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let cap = RE.captures(line).unwrap();
        let on: bool = cap[1].contains("on");
        let x_start = i32::from_str_radix(&cap[2], 10).unwrap();
        let x_end = i32::from_str_radix(&cap[3], 10).unwrap() + 1;
        let y_start = i32::from_str_radix(&cap[4], 10).unwrap();
        let y_end = i32::from_str_radix(&cap[5], 10).unwrap() + 1;
        let z_start = i32::from_str_radix(&cap[6], 10).unwrap();
        let z_end = i32::from_str_radix(&cap[7], 10).unwrap() + 1;
        Instruction {
            on,
            x_range: x_start..x_end,
            y_range: y_start..y_end,
            z_range: z_start..z_end
        }
    }

    fn to_box(&self) -> Box {
        Box::new(self.x_range.clone(),
                 self.y_range.clone(),
                 self.z_range.clone())
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Box {
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>
}

impl Box {

    fn new(x_range: Range<i32>, y_range: Range<i32>, z_range: Range<i32>) -> Box {
        Box {
            x_range,
            y_range,
            z_range
        }
    }

    fn intersects(&self, other: &Box) -> bool {
        range_intersects(&self.x_range, &other.x_range)
        && range_intersects(&self.y_range, &other.y_range)
        && range_intersects(&self.z_range, &other.z_range)
    }

    fn intersection(&self, other: &Box) -> Box {
        Box::new(range_intersection(&self.x_range, &other.x_range),
                 range_intersection(&self.y_range, &other.y_range),
                 range_intersection(&self.z_range, &other.z_range))
    }

    fn uncovered_areas(&self, covered_region: &Box) ->  Vec<Box> {
        self.minus(&self.intersection(covered_region))
    }

    fn minus(&self, other: &Box) -> Vec<Box> {
        let mut answer: Vec<Box> = Vec::new();

        // Everything to the front,

        let to_the_left_x = self.x_range.start..other.x_range.start;
        if !to_the_left_x.is_empty() {
            let to_push = Box::new(to_the_left_x, self.y_range.clone(), self.z_range.clone());
            // println!("Pushing front {:?}: s:{:?} ..", to_push, to_push.region());
            answer.push(to_push);
        }

        // Everything to the back,
        let to_the_right_x = other.x_range.end..self.x_range.end;
        if !(to_the_right_x.is_empty()) {
            let to_push = Box::new(to_the_right_x, self.y_range.clone(), self.z_range.clone());
            // println!("Pushing back {:?}: s:{:?} ..", to_push, to_push.region());
            answer.push(to_push)
        }

        // Everything below
        let below_y_range = self.y_range.start..other.y_range.start;
        if !(below_y_range.is_empty()) {
            let to_push = Box::new(other.x_range.clone(), below_y_range, self.z_range.clone());
            // println!("Pushing below {:?}: s:{:?} ..", to_push, to_push.region());
            answer.push(to_push)
        }

        // Everything above
        let above_y_range = other.y_range.end..self.y_range.end;
        if !(above_y_range.is_empty()) {
            let to_push = Box::new(other.x_range.clone(), above_y_range, self.z_range.clone());
            // println!("Pushing above {:?}: s:{:?} ..", to_push, to_push.region());
            answer.push(to_push)
        }

        // AE: Left and right might be swapped here I'm to druk to care and I can't think in more than 2 dimensions anyway

        // Everything to the left,
        let left_z_range = self.z_range.start..other.z_range.start;
        if !(left_z_range.is_empty()) {
            let to_push = Box::new(other.x_range.clone(), other.y_range.clone(), left_z_range);
            // println!("Pushing left {:?}: s:{:?} ..", to_push, to_push.region());
            answer.push(to_push)
        }

        // Everything to the right
        let right_z_range = other.z_range.end..self.z_range.end;
        if !(right_z_range.is_empty()) {
            let to_push = Box::new(other.x_range.clone(), other.y_range.clone(), right_z_range);
            // println!("Pushing right {:?}: s:{:?} ..", to_push, to_push.region());
            answer.push(to_push)
        }

        answer

    }

    fn region(&self) -> u64 {
        (self.x_range.len() as u64) * (self.y_range.len() as u64) * (self.z_range.len() as u64)
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn range_intersection(left: &Range<i32>, right: &Range<i32>) -> Range<i32> {
    let start = std::cmp::max(left.start, right.start);
    let end = std::cmp::min(left.end, right.end);
    start..end
}

fn range_differences(total: &Range<i32>, strip:  &Range<i32>) -> Vec<Range<i32>> {
    let mut answer : Vec<Range<i32>> = Vec::new();
    answer.push(total.start..(strip.start + 1));
    answer.push(strip.end..total.end);
    answer
}

fn range_intersects(left: &Range<i32>, right: &Range<i32>) -> bool {
    left.contains(&(right.end - 1)) || left.contains(&right.start)
    || right.contains( &(left.end -1)) || right.contains(&left.start)
}

fn main() -> io::Result<()> {
    let problem = read_lines(&env::args().collect::<Vec<String>>()[1]).unwrap();
    println!("{:?} many cubes are on", problem.solve_1());
    println!("{:?} many cubes are on", problem.solve_2());
    Ok(())
}

fn read_lines(filename: &String) -> io::Result<Problem> {
    let lines = BufReader::new(File::open(filename)?).lines();
    let instructions = lines.map(|line|{
        Instruction::new(line.unwrap().as_str())
    }).collect();
    Ok(Problem{ instructions })
}
