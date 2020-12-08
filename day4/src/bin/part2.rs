extern crate regex;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1]; // this will panic if there's no argument which is fine for this

    let file = File::open(filename).expect("couldn't open file");

    let mut passport: Passport = Default::default();
    let mut passports = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            passports.push(passport);
            passport = Default::default();
        } else {
            // lines look like foo:1234 bar:1234
            for field in line.split_whitespace() {
                // fields look like foo:1234
                let mut foo = field.split(':');
                let (field_name, field_value) = (foo.nth(0).unwrap(), foo.nth(0).unwrap());
                match field_name {
                    "byr" => passport.byr = Some(field_value.to_string()),
                    "iyr" => passport.iyr = Some(field_value.to_string()),
                    "eyr" => passport.eyr = Some(field_value.to_string()),
                    "hgt" => passport.hgt = Some(field_value.to_string()),
                    "hcl" => passport.hcl = Some(field_value.to_string()),
                    "ecl" => passport.ecl = Some(field_value.to_string()),
                    "pid" => passport.pid = Some(field_value.to_string()),
                    "cid" => passport.cid = Some(field_value.to_string()),
                    _ => {}
                }
            }
        }
    }
    passports.push(passport);

    println!(
        "number of valid passports: {}",
        passports
            .iter()
            .filter(|&passport| passport.is_valid())
            .count()
    );
}

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
            && self.validate_cid()
    }

    fn validate_byr(&self) -> bool {
        match &self.byr {
            Some(byr) => match byr.parse() {
                Ok(year) => 2002 >= year && year >= 1920,
                _ => false,
            },
            None => false,
        }
    }
    fn validate_iyr(&self) -> bool {
        match &self.iyr {
            Some(iyr) => match iyr.parse() {
                Ok(year) => 2020 >= year && year >= 2010,
                _ => false,
            },
            None => false,
        }
    }
    fn validate_eyr(&self) -> bool {
        match &self.eyr {
            Some(eyr) => match eyr.parse() {
                Ok(year) => 2030 >= year && year >= 2020,
                _ => false,
            },
            None => false,
        }
    }
    fn validate_hgt(&self) -> bool {
        match &self.hgt {
            Some(hgt) => {
                let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
                match re.captures(&hgt) {
                    Some(captures) => {
                        let value = captures.get(1).unwrap().as_str().parse().unwrap();
                        let units = captures.get(2).unwrap().as_str();

                        println!("value: {}, units: {}", value, units);
                        match units {
                            "cm" => 193 >= value && value >= 150,
                            "in" => 76 >= value && value >= 59,
                            _ => false,
                        }
                    }
                    None => {
                        println!("failed to parse!",);
                        false
                    }
                }
            }
            None => false,
        }
    }
    fn validate_hcl(&self) -> bool {
        match &self.hcl {
            Some(hcl) => {
                let re = Regex::new(r"^#([0-9a-fA-F]{6})$").unwrap();
                match re.captures(&hcl) {
                    Some(_) => true,
                    None => false,
                }
            }
            None => false,
        }
    }
    fn validate_ecl(&self) -> bool {
        match &self.ecl {
            Some(ecl) => {
                let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                valid.contains(&ecl.as_str())
            }
            None => false,
        }
    }
    fn validate_pid(&self) -> bool {
        match &self.pid {
            Some(pid) => {
                let re = Regex::new(r"^\d{9}$").unwrap();
                re.is_match(pid.as_str())
            }
            None => false,
        }
    }
    fn validate_cid(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_byr() {
        let mut passport: Passport = Default::default();
        passport.byr = Some("1200".to_string());
        assert_eq!(false, passport.validate_byr());
        passport.byr = Some("3200".to_string());
        assert_eq!(false, passport.validate_byr());
        passport.byr = Some("1920".to_string());
        assert_eq!(true, passport.validate_byr());
        passport.byr = Some("2002".to_string());
        assert_eq!(true, passport.validate_byr());
        passport.byr = Some("1964".to_string());
        assert_eq!(true, passport.validate_byr());
        passport.byr = Some("lolwut".to_string());
        assert_eq!(false, passport.validate_byr());
    }
    #[test]
    fn test_valid_iyr() {
        let mut passport: Passport = Default::default();
        passport.iyr = Some("1200".to_string());
        assert_eq!(false, passport.validate_iyr());
        passport.iyr = Some("3200".to_string());
        assert_eq!(false, passport.validate_iyr());
        passport.iyr = Some("2010".to_string());
        assert_eq!(true, passport.validate_iyr());
        passport.iyr = Some("2020".to_string());
        assert_eq!(true, passport.validate_iyr());
        passport.iyr = Some("2015".to_string());
        assert_eq!(true, passport.validate_iyr());
        passport.iyr = Some("lolwut".to_string());
        assert_eq!(false, passport.validate_iyr());
    }
    #[test]
    fn test_valid_eyr() {
        let mut passport: Passport = Default::default();
        passport.eyr = Some("1200".to_string());
        assert_eq!(false, passport.validate_eyr());
        passport.eyr = Some("3200".to_string());
        assert_eq!(false, passport.validate_eyr());
        passport.eyr = Some("2020".to_string());
        assert_eq!(true, passport.validate_eyr());
        passport.eyr = Some("2030".to_string());
        assert_eq!(true, passport.validate_eyr());
        passport.eyr = Some("2025".to_string());
        assert_eq!(true, passport.validate_eyr());
        passport.eyr = Some("lolwut".to_string());
        assert_eq!(false, passport.validate_eyr());
    }
    #[test]
    fn test_valid_hgt() {
        let mut passport: Passport = Default::default();
        passport.hgt = Some("10cm".to_string());
        assert_eq!(false, passport.validate_hgt());
        passport.hgt = Some("1000cm".to_string());
        assert_eq!(false, passport.validate_hgt());
        passport.hgt = Some("150cm".to_string());
        assert_eq!(true, passport.validate_hgt());
        passport.hgt = Some("193cm".to_string());
        assert_eq!(true, passport.validate_hgt());
        passport.hgt = Some("165cm".to_string());
        assert_eq!(true, passport.validate_hgt());
        passport.hgt = Some("10in".to_string());
        assert_eq!(false, passport.validate_hgt());
        passport.hgt = Some("1000in".to_string());
        assert_eq!(false, passport.validate_hgt());
        passport.hgt = Some("59in".to_string());
        assert_eq!(true, passport.validate_hgt());
        passport.hgt = Some("76in".to_string());
        assert_eq!(true, passport.validate_hgt());
        passport.hgt = Some("69in".to_string());
        assert_eq!(true, passport.validate_hgt());
        passport.hgt = Some("42".to_string());
        assert_eq!(false, passport.validate_hgt());
        passport.hgt = Some("lolcm".to_string());
        assert_eq!(false, passport.validate_hgt());
        passport.hgt = Some("lolin".to_string());
        assert_eq!(false, passport.validate_hgt());
    }
    #[test]
    fn test_valid_hcl() {
        let mut passport: Passport = Default::default();
        passport.hcl = Some("#abcdef".to_string());
        assert_eq!(true, passport.validate_hcl());
        passport.hcl = Some("#ABCDEF".to_string());
        assert_eq!(true, passport.validate_hcl());
        passport.hcl = Some("#123456".to_string());
        assert_eq!(true, passport.validate_hcl());
        passport.hcl = Some("#12q456".to_string());
        assert_eq!(false, passport.validate_hcl());
        passport.hcl = Some("#abcdeff".to_string());
        assert_eq!(false, passport.validate_hcl());
        passport.hcl = Some("abcdef".to_string());
        assert_eq!(false, passport.validate_hcl());
        passport.hcl = Some("123456".to_string());
        assert_eq!(false, passport.validate_hcl());
    }
    #[test]
    fn test_valid_ecl() {
        let mut passport: Passport = Default::default();

        passport.ecl = Some("amb".to_string());
        assert_eq!(true, passport.validate_ecl());
        passport.ecl = Some("farts".to_string());
        assert_eq!(false, passport.validate_ecl());
    }
    #[test]
    fn test_valid_pid() {
        let mut passport: Passport = Default::default();
        passport.pid = Some("123456789".to_string());
        assert_eq!(true, passport.validate_pid());
        passport.pid = Some("122345".to_string());
        assert_eq!(false, passport.validate_pid());
        passport.pid = Some("1234567890".to_string());
        assert_eq!(false, passport.validate_pid());
        passport.pid = Some("123abc789".to_string());
        assert_eq!(false, passport.validate_pid());
    }
    #[test]
    fn test_valid_cid() {
        let passport: Passport = Default::default();
        assert_eq!(true, passport.validate_cid());
    }
}
