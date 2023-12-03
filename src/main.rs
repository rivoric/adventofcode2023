pub mod day01;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DEBUG: bool = false;

fn main() {
    let file_path = "input/day01live.txt";

    if let Ok(lines) = read_lines(file_path) {
        let (parta, partb) = day01::day01(lines, DEBUG);
        println!("Part A: {}", parta);
        println!("Part B: {}", partb);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
