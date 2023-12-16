use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub(crate) fn solve() {
    if let Ok(lines) = read_lines("src/day1/input.txt") {
        let mut sum = 0;

        for line in lines {
            if let Ok(content) = line {
                let chars: Vec<char> = content.chars().collect();
                let mut iter = chars.iter()
                    .filter(|c| c.is_ascii_digit());

                let first = iter.next();
                let mut last = iter.last();
                if last.is_none() {
                    last = first;
                }

                match (first, last) {
                    (Some(first), Some(last)) => {
                        let mut n_str = String::new();
                        n_str.push(*first);
                        n_str.push(*last);

                        match n_str.parse::<i32>() {
                            Ok(v) => sum += v,
                            Err(e) => panic!("could not parse as int! {}", e)
                        }
                    }
                    _ => {}
                }
            }
        }

        println!("Calibration Sum: {}", sum)
    }
}