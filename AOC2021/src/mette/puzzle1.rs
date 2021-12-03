use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
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
