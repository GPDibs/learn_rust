use Movement::{Up, Down, Forward};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Submarine {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Submarine {
    fn moves(&mut self, movement :&Movement) -> () {
        match movement {
            Up(x) => {self.aim -= x},
            Down(x) => {self.aim += x},
            Forward(x) => {self.horizontal += x; self.depth += self.aim * x},
            _ => println!("unexpected"),
        };
    }
}

#[derive(Debug)]
enum Movement {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn main() {
    let list = transform_instructions();
  
    println!("this is main starting");
  
    puzzle1(&list);
    puzzle2(&list);
}

fn transform_instructions() -> Vec<Movement> {
    let filename = String::from("inputp3p4.txt");
    let file = File::open(filename).expect("file cannot be read");
    let reader = BufReader::new(file);        

    let mut result: Vec<Movement> = Vec::new();

    for perline in reader.lines(){
        match perline {
            Ok(line) => { let splits:Vec<&str> = line.split_whitespace().collect();
                result.push(match splits[0] {        
                    "forward" => Movement::Forward(splits[1].parse::<u32>().expect("parsing error")),
                    "down" => Movement::Down(splits[1].parse::<u32>().expect("parsing error")),
                    "up" => Movement::Up(splits[1].parse::<u32>().expect("parsing error")),
                    _ => continue,
                });
            },
            _ => continue,
        }
    }
    
    result
}

fn puzzle1(vec:&Vec<Movement>) { 
    let mut sub = Submarine {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for mov in vec {
        sub.moves(mov);        
    }

    println!("puzzle 1 horizontal: {}, puzzle 1 depth: {}", sub.horizontal, sub.depth);
    println!("final answer: {}", sub.horizontal * sub.depth);
}

fn puzzle2(vec:&Vec<Movement>) {  
    println!("puzzle 2:");
    
    let mut sub2 = Submarine {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for mov in vec {
        sub2.moves(mov);
    }
    
    println!("aim: {}", sub2.aim);
    println!("final answer: {}", sub2.horizontal * sub2.depth);
    
}










