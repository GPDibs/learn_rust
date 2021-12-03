// src/bin/r_1.rs -> src/reinier/day1.rs

use std::fs;
use std::slice::Windows;

fn main() {
    puzzle1();
    puzzle2(); 
}

fn puzzle1() {
    let filename :String = String::from("input/day1_1.txt");
    let input_string = fs::read_to_string(filename).expect("file reading went wrong");
    let result = input_string
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
        .expect("oop")
        .windows(2)
        .filter(|w| w[1] > w[0])
        .collect::<Vec<_>>()
        .len();



    println!("{:?}", result);
}


fn puzzle2() {
    let filename :String = String::from("input/day1_1.txt");
    let input_string = fs::read_to_string(filename).expect("file reading went wrong");
    let result = input_string
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
        .expect("oop")
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .collect::<Vec<_>>()
        .len();

    println!("{:?}", result);
}
