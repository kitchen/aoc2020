extern crate regex;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1]; // this will panic if there's no argument which is fine for this

    let file = File::open(filename).expect("couldn't open file");

    let mut num_good: usize = 0;
    for line in io::BufReader::new(file).lines() {
        match PolicySpec::from_line(line.unwrap()) {
            Ok((spec, password)) => {
                if spec.check_password(password) {
                    num_good += 1;
                }
            }
            Err(error) => {
                println!("error parsing line {}", error);
            }
        }
    }
    println!("good passwords: {}", num_good);
}

#[derive(Debug, PartialEq)]
struct PolicySpec {
    lower: usize,
    upper: usize,
    letter: char,
}

impl PolicySpec {
    fn from_line(line: String) -> Result<(PolicySpec, String), String> {
        let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
        match re.captures(&line) {
            Some(captures) => {
                let lower = captures.get(1).unwrap().as_str().parse().unwrap();
                let upper = captures.get(2).unwrap().as_str().parse().unwrap();
                let letter = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
                let password = captures.get(4).unwrap().as_str();

                Ok((
                    PolicySpec {
                        lower: lower,
                        upper: upper,
                        letter: letter,
                    },
                    password.to_string(),
                ))
            }
            None => Err(format!("regex didn't match line: {}", line)),
        }
    }

    fn check_password(&self, password: String) -> bool {
        let mut num_found: usize = 0;
        for character in password.chars() {
            if character == self.letter {
                num_found += 1;
            }
        }
        if num_found <= self.upper && num_found >= self.lower {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_spec() {
        assert_eq!(
            Ok((
                PolicySpec {
                    lower: 1,
                    upper: 3,
                    letter: 'a'
                },
                "abcde".to_string()
            )),
            PolicySpec::from_line("1-3 a: abcde".to_string())
        );
        assert_eq!(
            Ok((
                PolicySpec {
                    lower: 1,
                    upper: 3,
                    letter: 'b'
                },
                "cdefg".to_string()
            )),
            PolicySpec::from_line("1-3 b: cdefg".to_string())
        );
        assert_eq!(
            Ok((
                PolicySpec {
                    lower: 2,
                    upper: 9,
                    letter: 'c'
                },
                "ccccccccc".to_string()
            )),
            PolicySpec::from_line("2-9 c: ccccccccc".to_string())
        );
    }

    #[test]
    fn test_check_password() {
        assert_eq!(
            true,
            PolicySpec {
                lower: 1,
                upper: 3,
                letter: 'a'
            }
            .check_password("abcde".to_string())
        );
        assert_eq!(
            false,
            PolicySpec {
                lower: 1,
                upper: 3,
                letter: 'b'
            }
            .check_password("cdefg".to_string())
        );
        assert_eq!(
            true,
            PolicySpec {
                lower: 2,
                upper: 9,
                letter: 'c'
            }
            .check_password("ccccccccc".to_string())
        );
    }
}
