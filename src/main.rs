use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DEBUG: bool = false;

fn main() {
    let file_path = "input/day01live.txt";
    let mut parta: u32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for readline in lines {
            if let Ok(line) = readline {
                parta += aoc::first_last_digit(&line, DEBUG);
            }
        }
        println!("Part A: {}", parta);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
