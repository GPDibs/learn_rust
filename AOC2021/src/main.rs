use Movement::{Up, Down, Forward};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Submarine {
    horizontal: u32,
    depth: u32,
}

impl Submarine {
    fn moves(&mut self, movement:Movement) -> () {
        match movement {
            Up(x) => self.depth -= x,
            Down(x) => self.depth += x,
            Forward(x) => self.horizontal += x,
        };
    }
}

enum Movement {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn main() {
    println!("this is main starting");
    puzzle1();
    puzzle2();
}

fn transform_instructions() -> Vec<Movement> {
    let filename = String::from("inputp3p4.txt");
    let file = File::open(filename).expect("file cannot be read");
    let reader = BufReader::new(file);        

    let mut result: Vec<Movement> = Vec::new();

    for line in reader.lines(){
        let splits = line.unwrap().split_whitespace();
        result.push(match splits[0]{
            "forward" => Movement::Forward(splits[1].parse::<u32>().expect("parsing error")),
            "down" => Movement::Down(splits[1].parse::<u32>().expect("parsing error")),
            "up" => Movement::Up(splits[1].parse::<u32>().expect("parsing error")),                    
        });
    }
    
    result
}

fn puzzle1() { 
    let mut sub = Submarine {
        horizontal: 0,
        depth: 0,
    };
}

fn puzzle2() {  
    println!("puzzle 2");
}

