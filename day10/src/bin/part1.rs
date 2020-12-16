use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // this will panic if there's no argument which is fine for this
    let file = File::open(filename).expect("couldn't open file");

    let mut adapters: HashSet<i64> = HashSet::new();
    for line in io::BufReader::new(file).lines() {
        adapters.insert(line.unwrap().parse().unwrap());
    }
    adapters.insert(0);

    let max_adapter = adapters.iter().max().unwrap();
    let mut current_jolts = max_adapter + 3;
    let mut accumulator: HashMap<i64, usize> = HashMap::new();
    accumulator.insert(1, 0);
    accumulator.insert(2, 0);
    accumulator.insert(3, 0);

    let mut adapters_sorted: Vec<i64> = adapters.into_iter().collect::<Vec<i64>>();
    adapters_sorted.sort();
    adapters_sorted.reverse();
    for adapter in adapters_sorted {
        if adapter == 0 {
            println!("got to adapter 0");
        }
        let diff = current_jolts - adapter;
        if diff <= 3 {
            accumulator.entry(diff).and_modify(|e| *e += 1);
            current_jolts = adapter
        } else {
            break;
        }
    }

    println!(
        "1: {}, 3: {}, 1 * 3: {}",
        accumulator.get(&1).unwrap(),
        accumulator.get(&3).unwrap(),
        accumulator.get(&1).unwrap() * accumulator.get(&3).unwrap()
    );
}
