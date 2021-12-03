use std::fs;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    puzzle2();
}

fn puzzle1() {
    let file = File::open("/inputfiles/inputp1p2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut count:u32 = 0;
    let mut previous: u32 = 0;

    for line in reader.lines(){
        let current:u32 = line.expect("error current").trim().parse().unwrap();      

        if current > previous {
            count = count + 1;
        }

        // update previous
        previous = current;
    }

    let total = count - 1; // minus first one
    println!("{}", total);

}

fn puzzle2() {
    let filename: String = String::from("inputp1p2.txt");    
    let input_as_string = fs::read_to_string(filename).expect("cannot read file");

    // make growing list vector with lines as items as u32
    let list_of_data = input_as_string
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>,_>>()        
        .expect("error")
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .collect::<Vec<_>>()
        .len();

    println!("{:?}", list_of_data);
}




