use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1]; // this will panic if there's no argument which is fine for this

    let file = File::open(filename).expect("couldn't open file");

    let mut total = 0;
    let mut group_answers = HashMap::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            total += group_answers.keys().count();
            println!(
                "{} group votes: {}",
                group_answers.keys().count(),
                group_answers.keys().collect::<String>()
            );
            group_answers = HashMap::new();
        } else {
            for answer in line.chars() {
                group_answers.insert(answer, true);
            }
        }
    }
    total += group_answers.keys().count();
    println!(
        "{} group votes: {}",
        group_answers.keys().count(),
        group_answers.keys().collect::<String>()
    );

    println!("total: {}", total);
}
