extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // this will panic if there's no argument which is fine for this
    let file = File::open(filename).expect("couldn't open file");

    let mut bags: HashMap<String, Bag> = HashMap::new();

    let line_re = Regex::new(r"^(.*?) bags contain (.*)$").unwrap();
    let bag_re = Regex::new(r"^ ?(\d+) (.*?) bags?[.,]?").unwrap();

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let line_captures = line_re.captures(line.as_str()).unwrap();
        let line_bag_name = &line_captures[1];
        bags.entry(line_bag_name.to_string())
            .or_insert(Default::default());

        println!("bag: {} contains:", line_bag_name);
        for contained_bag_entry in line_captures[2].split(",") {
            println!("\t DEBUG: {}", contained_bag_entry);
            match bag_re.captures(contained_bag_entry) {
                Some(matches) => {
                    let num_contained = matches[1].to_string().parse().unwrap();
                    let contained_bag_name = &matches[2];
                    println!("\t{} {}", num_contained, contained_bag_name);
                    let contained_bag = bags
                        .entry(contained_bag_name.to_string())
                        .or_insert(Default::default());
                    contained_bag.contained_by.insert(line_bag_name.to_string());
                    bags.entry(line_bag_name.to_string()).and_modify(|bag| {
                        bag.contains
                            .insert(contained_bag_name.to_string(), num_contained);
                    });
                }
                None => {}
            }
        }
    }

    println!("bags: {:#?}", bags);

    let contained_by = find_containers_of(&bags, "shiny gold".to_string());
    println!(
        "shiny gold can be contained by these bag types: {:#?}",
        // contained_by.count(),
        contained_by
    );
}

fn find_containers_of(bags: &HashMap<String, Bag>, bag_name: String) -> HashSet<String> {
    println!("finding bags that contain {}", bag_name);
    let mut contained_by = HashSet::new();
    let bag = bags.get(&bag_name).unwrap();
    for container in bag.contained_by.iter() {
        contained_by.insert(container.to_string());
        for container_contained_by in find_containers_of(&bags, container.to_string()) {
            contained_by.insert(container_contained_by.to_string());
        }
    }
    contained_by
}

#[derive(Default, Debug)]
struct Bag {
    contains: HashMap<String, usize>,
    contained_by: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(true, true);
    }
}
