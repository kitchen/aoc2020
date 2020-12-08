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
        match self {
            Passport {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                cid: _,
            } => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let mut passport: Passport = Default::default();
        assert_eq!(false, passport.is_valid());

        passport.byr = Some("foo".to_string());
        passport.iyr = Some("foo".to_string());
        passport.eyr = Some("foo".to_string());
        passport.hgt = Some("foo".to_string());
        passport.hcl = Some("foo".to_string());
        passport.ecl = Some("foo".to_string());
        passport.pid = Some("foo".to_string());

        assert_eq!(true, passport.is_valid());
        passport.cid = Some("foo".to_string());
        assert_eq!(true, passport.is_valid());
    }
}
