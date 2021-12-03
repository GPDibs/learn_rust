use Movement::{Up, Down, Forward};

use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug)]
struct Submarine {

    horizontal :i32,
    depth :i32,
    aim :i32
}

impl Submarine {
    fn moves(&mut self, movement :&Movement) -> () {
        match movement {
            Up(x) => self.depth -= x,
            Down(x) => self.depth += x,
            Forward(x) => self.horizontal += x,
        };

    }

    fn moves2(&mut self, movement :&Movement) -> () {
        match movement {
            Up(x) => self.aim -= x,
            Down(x) => self.aim += x,
            Forward(x) => {self.horizontal += x; self.depth += self.aim*x;},
        };
    }
}

fn main() {
    let list = transform_instructions();
    println!("main");
    puzzle1(&list);
    puzzle2(&list);
}

fn puzzle1(vec :&Vec<Movement>) -> () {
    let mut sub = Submarine {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for mov in vec {
        sub.moves(&mov);
    };

    println!("puzzle1: {:?}", sub.horizontal * sub.depth);
}

fn puzzle2(vec :&Vec<Movement>) -> () {
    let mut sub = Submarine {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for mov in vec {
        sub.moves2(&mov);
    };

    println!("puzzle2: {:?}", sub.horizontal * sub.depth);
}

fn transform_instructions() -> Vec<Movement> {
    let filename = String::from("input/day2_1.txt");

    let file = File::open(filename).expect("IOException");

    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for errline in reader.lines() {
        match errline {
            Ok(line) => { let splits :Vec<&str> = line.split_whitespace().collect();
                result.push(match splits[0] {
                    "forward" => Movement::Forward(splits[1].parse::<i32>().expect("parsing fault")),
                    "down" => Movement::Down(splits[1].parse::<i32>().expect("parsing fault")),
                    "up" => Movement::Up(splits[1].parse::<i32>().expect("parsing faukt")),
                    _ => panic!("unexpected input"),
                });},
             _ => continue,
        }
    }

    result
}
