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
        for j in i..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return Ok(expenses[i] * expenses[j]);
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
        assert_eq!(Ok(10 * 2010), find_product_2020_sum(vec!(10, 2010)));
        assert_eq!(
            Ok(5 * 2015),
            find_product_2020_sum(vec!(30, 40, 5, 2000, 1952, 2015, 1234))
        );
        assert_eq!(
            Ok(514579),
            find_product_2020_sum(vec![1721, 979, 366, 299, 675, 1456])
        );
    }
}
