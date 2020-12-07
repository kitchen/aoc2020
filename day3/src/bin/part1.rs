use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1]; // this will panic if there's no argument which is fine for this

    let file = File::open(filename).expect("couldn't open file");

    let mut trees = 0;
    let mut y = 0;
    let step_x = 3;
    let step_y = 1;
    let mut want_x = step_x;
    let mut want_y = step_y;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        let max_x = line.chars().count();
        if y > 0 && y == want_y {
            match line.chars().nth(want_x) {
                Some('#') => trees += 1,
                _ => {}
            };

            want_x += step_x;
            if want_x >= max_x {
                want_x -= max_x;
            }

            want_y += step_y;
        }
        y += 1;
    }
    println!("trees: {}", trees);
}
