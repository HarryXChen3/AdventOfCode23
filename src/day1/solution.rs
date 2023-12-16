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

fn next_word(
    n: u32,
    content: &String,
    word: &&str,
    last_word: Option<(usize, u32)>,
    find_trailing: bool
) -> Option<(usize, u32)> {
    let found = if find_trailing {
        content.rfind(word)
    } else {
        content.find(word)
    };

    if let Some(find) = found {
        if let Some((last, _)) = last_word {
            if (find_trailing && find > last) || (!find_trailing && find < last) {
                return Some((find, n));
            }
        } else {
             return Some((find, n));
        }
    }

    None
}

fn combine_chars(first_char: char, last_char: char) -> i32 {
    let mut n_str = String::new();
    n_str.push(first_char);
    n_str.push(last_char);

    match n_str.parse::<i32>() {
        Ok(v) => v,
        Err(e) => panic!("could not parse {} as int! {}", n_str, e),
    }
}

const WORD_DIGITS: &'static [&'static str] =
    &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub(crate) fn solve() {
    if let Ok(lines) = read_lines("src/day1/input.txt") {
        let mut digit_only_sum = 0;
        let mut combined_sum = 0;

        for line in lines {
            if let Ok(content) = line {
                let mut digit_iter = content.chars().enumerate()
                    .filter(|(_, c)| c.is_ascii_digit());

                let first_digit = digit_iter.next();
                let mut last_digit = digit_iter.last();
                if last_digit.is_none() {
                    last_digit = first_digit;
                }

                match (first_digit, last_digit) {
                    (Some((_, first_char)), Some((_, last_char))) => {
                        let mut n_str = String::new();
                        n_str.push(first_char);
                        n_str.push(last_char);

                        match n_str.parse::<i32>() {
                            Ok(v) => digit_only_sum += v,
                            Err(e) => panic!("could not parse {} as int! {}", n_str, e),
                        }
                    }
                    _ => {}
                }

                let mut leading_word: Option<(usize, u32)> = None;
                let mut trailing_word: Option<(usize, u32)> = None;
                for (i, word) in WORD_DIGITS.iter().enumerate() {
                    let n = i as u32;
                    let next_leading_word = next_word(n, &content, word, leading_word, false);
                    if next_leading_word.is_some() {
                        leading_word = next_leading_word;
                    }

                    let next_trailing_word = next_word(n, &content, word, trailing_word, true);
                    if next_trailing_word.is_some() {
                        trailing_word = next_trailing_word;
                    }
                }

                match (first_digit, last_digit, leading_word, trailing_word) {
                    (
                        Some((f_digit_index, f_digit_char)),
                        Some((l_digit_index, l_digit_char)),
                        Some((f_word_index, f_word_n)),
                        Some((l_word_index, l_word_n))
                    ) => {
                        let n = combine_chars(
                            if f_word_index < f_digit_index {
                                char::from_digit(f_word_n, 10).unwrap()
                            } else {
                                f_digit_char
                            },
                            if l_word_index > l_digit_index {
                                char::from_digit(l_word_n, 10).unwrap()
                            } else {
                                l_digit_char
                            }
                        );

                        combined_sum += n;
                    },
                    (
                        Some((_, first_char)),
                        Some((_, last_char)),
                        None,
                        None
                    ) => {
                        combined_sum += combine_chars(first_char, last_char);
                    },
                    (
                        None,
                        None,
                        Some((_, first_n)),
                        Some((_, last_n))
                    ) => {
                        let n = combine_chars(
                            char::from_digit(first_n, 10).unwrap(),
                            char::from_digit(last_n, 10).unwrap()
                        );

                        combined_sum += n;
                    }
                    _ => {}
                }
            }
        }

        println!("Part 1: {}", digit_only_sum);
        println!("Part 2: {}", combined_sum);
    }
}