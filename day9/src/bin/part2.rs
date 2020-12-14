use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1]; // this will panic if there's no argument which is fine for this

    let file = File::open(filename).expect("couldn't open file");
    let search: i64 = args[2].parse().unwrap();

    let mut numbers: Vec<i64> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let num: i64 = line.unwrap().parse().unwrap();
        let numbers_len = numbers.len();
        let mut sum = num;
        for i in (0..numbers_len).rev() {
            sum += numbers[i];
            if sum == search {
                let mut smallest = num;
                let mut largest = num;
                for j in (i..numbers_len) {
                    if numbers[j] > largest {
                        largest = numbers[j];
                    }
                    if numbers[j] < smallest {
                        smallest = numbers[j];
                    }
                }
                panic!(
                    "found it! {} -> {}: {} + {} = {}",
                    num,
                    numbers[i],
                    smallest,
                    largest,
                    smallest + largest
                );
            }
        }
        numbers.push(num);
    }
    println!("never found it ...");
}
