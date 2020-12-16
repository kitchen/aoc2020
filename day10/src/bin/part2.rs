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

    let mut adapters_sorted: Vec<i64> = adapters.clone().into_iter().collect::<Vec<i64>>();
    let mut seen_adapters: HashMap<i64, usize> = HashMap::new();
    adapters_sorted.sort();
    println!("{:?}", adapters);

    seen_adapters.insert(0, 1);
    for adapter in adapters_sorted {
        if adapter == 0 {
            seen_adapters.insert(0, 1);
            continue;
        }
        let mut total_paths = 0;
        for possible_child in (adapter - 3)..adapter {
            println!("checking possible child: {}", possible_child);
            if adapters.contains(&possible_child) {
                println!("\twe found a child! {}", possible_child);
                total_paths += seen_adapters[&possible_child];
            }
        }

        seen_adapters.insert(adapter, total_paths);
        println!("\tadapter: {}, total paths: {}", adapter, total_paths);
    }
}
