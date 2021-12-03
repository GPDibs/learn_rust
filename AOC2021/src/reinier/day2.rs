use Movement::{Up, Down, Forward, No};

use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
enum Movement {
    Up(u32),
    Down(u32),
    Forward(u32),
    No,
}

#[derive(Debug)]
struct Submarine {

    horizontal :u32,
    depth: u32,

}

impl Submarine {
    fn moves(&mut self, movement :Movement) -> () {
        match movement {
            Up(x) => self.depth -= x,
            Down(x) => self.depth += x,
            Forward(x) => self.horizontal += x,
            _ => println!("THATS UNEXPECTED!"),
        };

    }
}

fn main() {
    let list = transform_instructions();
    println!("main");
    puzzle1(list);
    puzzle2();
}

fn puzzle1(vec :Vec<Movement>) -> () {
    let mut sub = Submarine {
        horizontal: 0,
        depth: 0,
    };

    for mov in vec {
        sub.moves(mov);
    };

    println!("puzzle1: {:?}", sub);
}

fn puzzle2() {
    println!("puzzle2");
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
                    "forward" => Movement::Forward(splits[1].parse::<u32>().expect("parsing fault")),
                    "down" => Movement::Down(splits[1].parse::<u32>().expect("parsing fault")),
                    "up" => Movement::Up(splits[1].parse::<u32>().expect("parsing faukt")),
                    _ => Movement::No,
                });},
             _ => continue,
        }
    }

    result
}
