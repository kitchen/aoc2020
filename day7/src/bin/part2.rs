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

    println!(
        "child bags count: {}",
        count_child_bags(&bags, "shiny gold".to_string())
    );
}

fn count_child_bags(bags: &HashMap<String, Bag>, bag_name: String) -> usize {
    println!("counting child bags of {}", bag_name);
    let mut child_bag_count = 1;
    let bag = bags.get(&bag_name).unwrap();
    for child_bag in bag.contains.keys() {
        let multiplier = bag.contains.get(&child_bag.to_string()).unwrap();
        let child_bag_contains = count_child_bags(&bags, child_bag.to_string());
        println!(
            "\t{} {} which each contain {} bags, adding: {}",
            multiplier,
            child_bag,
            child_bag_contains,
            child_bag_contains * multiplier
        );

        child_bag_count += child_bag_contains * multiplier;
    }

    println!("\t returning {}", child_bag_count);
    child_bag_count
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
