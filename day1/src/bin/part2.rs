use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1]; // this will panic if there's no argument which is fine for this

    let expenses_file = File::open(filename).expect("couldn't open file");
    let mut expenses = Vec::new();
    for line in io::BufReader::new(expenses_file).lines() {
        let expense: u32 = line.unwrap().parse::<u32>().unwrap();
        expenses.push(expense);
    }

    println!("expenses: {:?}", expenses);
    match find_product_2020_sum(expenses) {
        Ok(product) => println!("{}", product),
        Err(error) => println!("error! {}", error),
    }
}

fn find_product_2020_sum(expenses: Vec<u32>) -> Result<u32, &'static str> {
    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            for k in (j + 1)..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    return Ok(expenses[i] * expenses[j] * expenses[k]);
                }
            }
        }
    }
    Err("didn't find any pairs that add up to 2020")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {
        assert_eq!(
            Ok(241861950),
            find_product_2020_sum(vec![1721, 979, 366, 299, 675, 1456])
        );
    }
}
