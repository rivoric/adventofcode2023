use std::fs::File;
use std::io;

const RADIX: u32 = 10;

pub fn day01 (input: io::Lines<io::BufReader<File>>, debug: bool) -> (u32, u32) {
    let mut parta: u32 = 0;
    let mut partb: u32 = 0;

        // Consumes the iterator, returns an (Optional) String
        for readline in input {
            if let Ok(line) = readline {
                parta += first_last_digit(&line, debug);
                partb += first_last_digit_or_word(&line, debug);
            }
        }

    (parta, partb)
}

fn first_last_digit(line: &str, debug: bool) -> u32 {
    let mut first_char: char = ' ';
    let mut curr_char: char = ' ';
    let mut found_first = false;

    for c in line.chars() {
        if c.is_digit(RADIX) {
            if found_first {
                curr_char = c;
            }
            else {
                first_char = c;
                curr_char = c;
                found_first = true;
            }
        }
    }

    if debug {
        println!("str {}; first digit: {} last digit: {}", line, first_char, curr_char);
    }

    if found_first {
        return first_char.to_digit(RADIX).unwrap() * 10 + curr_char.to_digit(RADIX).unwrap();
    }
    0
}

fn is_number_at(line: &str, position: usize, number_string: &str) -> bool {
    let numlen = number_string.chars().count();

    if line.chars().count() < position + numlen {
        return false;
    }

    &line[position..position+numlen] == number_string
}

fn first_last_digit_or_word(line: &str, debug: bool) -> u32 {
    let mut first_char: u32 = 0;
    let mut curr_char: u32 = 0;
    let mut found_first = false;
    // let mut skip: u32 = 0;

    for (i, c) in line.chars().enumerate() {
        // aoc is allowing the same letter to be in multiple numbers,
        // for example eightwo is counted both as 8 and 2
        // if skip > 0 {
        //     skip -= 1;
        //     continue
        // }

        let mut value: u32 = 0;
        match c {
            '1'..='9' => {
                // digit
                value = c.to_digit(RADIX).unwrap();
            }
            'e' => {
                // check for eight
                if is_number_at(line, i, "eight") {
                    value = 8;
                    // skip = 4;
                }
            }
            'f' => {
                // check for four, five
                if is_number_at(line, i, "four") {
                    value = 4;
                    // skip = 3;
                }
                else if is_number_at(line, i, "five") {
                    value = 5;
                    // skip = 3;
                }
            }
            'n' => {
                // check for nine
                if is_number_at(line, i, "nine") {
                    value = 9;
                    // skip = 3;
                }
            }
            'o' => {
                // check for one
                if is_number_at(line, i, "one") {
                    value = 1;
                    // skip = 2;
                }
            }
            's' => {
                // check for six, seven
                if is_number_at(line, i, "six") {
                    value = 6;
                    // skip = 2;
                }
                else if is_number_at(line, i, "seven") {
                    value = 7;
                    // skip = 4;
                }
            }
            't' => {
                // check for two, three
                if is_number_at(line, i, "two") {
                    value = 2;
                    // skip = 2;
                }
                else if is_number_at(line, i, "three") {
                    value = 3;
                    // skip = 4;
                }
            }
            _ => {
                // no need to do anything
            }
        }

        if value > 0 {
            if found_first {
                curr_char = value;
            }
            else {
                first_char = value;
                curr_char = value;
                found_first = true;
            }
        }
    }

    if debug {
        println!("str {}; first digit: {} last digit: {}", line, first_char, curr_char);
    }

    if found_first {
        return first_char * 10 + curr_char;
    }
    0
}
