use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("main starts here");
    puzzle1();
    puzzle2();
}

fn puzzle1() {
    println!("puzzle 1 is starting");
    
    let filename = String::from("inputp3p1.txt");
    let firstfile = File::open(&filename).expect("cannot open file error");
    let mut initialreader = BufReader::new(firstfile);
    let mut readcharacter = Vec::<u8>::new();    
    
    let mut amountnums: usize = 0;
    // iterate over first line, read until newline
    match initialreader.read_until(b'\n', &mut readcharacter){
        Ok(buffer) => amountnums = buffer,
        _ => println!("oops error"),
    };

    let mut result = vec![vec![0; 2]; amountnums - 1];

    let file = File::open(&filename).expect("cant open file error");
    let mut reader = BufReader::new(file);
    
    let mut linebuffer = Vec::<u8>::new();
   
    while reader.read_until(b'\n', &mut linebuffer).expect("reader failed") != 0 {
        let line_as_string = String::from_utf8(linebuffer).expect("line as string failed");
        let string: &str = &line_as_string[0..line_as_string.len() - 1];
       
        for (i, c) in string.chars().enumerate() {
            if c == '1' {
                result[i][1] += 1;
            } else {
                result[i][0] += 1;
            }
        }
        linebuffer = line_as_string.into_bytes();
        linebuffer.clear();
    }

    let mut gamma = String::from("");    
    let mut epsilon = String::from("");

    for number in 0..result.len() {
        if result[number][0] > result[number][1] {
            gamma.push_str("0");     
            epsilon.push_str("1");   
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }
    
    println!("the final answer: {}", isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap());
    
}

fn puzzle2() {
    println!("puzzle 2 is starting");
}
