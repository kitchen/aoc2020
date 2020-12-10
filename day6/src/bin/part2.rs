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
    let mut group_members = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            total += group_answers
                .values()
                .filter(|&n| *n == group_members)
                .count();
            println!(
                "group_members: {}, group_answers: {:?}",
                group_members, group_answers
            );
            group_answers = HashMap::new();
            group_members = 0;
        } else {
            for answer in line.chars() {
                match group_answers.get(&answer) {
                    Some(&count) => group_answers.insert(answer, count + 1),
                    None => group_answers.insert(answer, 1),
                };
            }
            group_members += 1;
        }
    }
    total += group_answers
        .values()
        .filter(|&n| *n == group_members)
        .count();

    println!("total: {}", total);
}
