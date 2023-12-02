pub fn first_last_digit(line: &str, debug: bool) -> u32 {
    const RADIX: u32 = 10;

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
