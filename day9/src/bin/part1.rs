use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1]; // this will panic if there's no argument which is fine for this

    let file = File::open(filename).expect("couldn't open file");
    let preamble: usize = args[2].parse().unwrap();

    let mut numbers: Vec<i64> = Vec::new();
    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        let num: i64 = line.unwrap().parse().unwrap();
        if i < preamble {
            numbers.push(num);
        } else {
            if !check_number(num, &numbers) {
                panic!(
                    "{} isn't a sum of one of the previous {} numbers: {:?}",
                    num, preamble, numbers
                );
            }
            numbers.push(num);
            numbers.remove(0);
        }
    }
}

fn check_number(num: i64, numbers: &Vec<i64>) -> bool {
    for i in 0..(numbers.len() - 1) {
        for j in i..(numbers.len()) {
            if num == numbers[i] + numbers[j] {
                return true;
            }
        }
    }

    false
}
