use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("failed to read file");
    let mut first = None;
    let mut last = None;
    let mut sum: u64 = 0;
    for line in contents.lines() {
        for c in line.chars() {
            if c.is_digit(10) {
                first = Some(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = Some(c);
                break;
            }
        }
        match first {
            Some(f) => {
                match last {
                    Some(l) => sum += format!("{f}{l}").parse::<u64>().unwrap(),
                    None => panic!("WHOA L"),
                }
            },
            None => panic!("WHOA"),
        }
    }
    println!("{sum}");
}
