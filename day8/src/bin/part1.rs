use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::string::ParseError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // this will panic if there's no argument which is fine for this
    let file = File::open(filename).expect("couldn't open file");

    let mut stack: Vec<Instruction> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        match line.parse() {
            Ok(Instruction::Invalid) => panic!("farts"),
            Ok(instruction) => stack.push(instruction),
            _ => panic!("double farts!"),
        }
    }

    let mut acc: i64 = 0;
    let mut pos = 0;
    let mut seen_pos: HashSet<usize> = HashSet::new();
    loop {
        if seen_pos.contains(&pos) {
            panic!(
                "we've already seen instruction {} -> {:?}, accumulator is: {}",
                pos,
                stack.get(pos).unwrap(),
                acc
            );
        }
        seen_pos.insert(pos);
        match stack.get(pos) {
            Some(instruction) => pos = instruction.execute(pos, &mut acc),
            None => panic!("stack overflow!"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop,
    // alternatively, return Err, but I could not for the life of me make that work
    Invalid,
}

impl Instruction {
    fn execute(&self, pos: usize, acc: &mut i64) -> usize {
        match self {
            Instruction::Acc(value) => {
                *acc += value;
                pos + 1
            }
            Instruction::Jmp(value) => (pos as i64 + value) as usize,
            Instruction::Nop => pos + 1,
            _ => unimplemented!(),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let instruction = iter.next();
        let value_str = iter.next();
        let remaining = iter.next();
        match (instruction, value_str, remaining) {
            (Some(instruction), Some(value_str), None) => {
                let value = match value_str.parse() {
                    Ok(value) => value,
                    // alternatively, return Err, but I could not for the life of me make that work
                    Err(_) => return Ok(Instruction::Invalid),
                };
                match instruction {
                    "acc" => Ok(Instruction::Acc(value)),
                    "jmp" => Ok(Instruction::Jmp(value)),
                    "nop" => Ok(Instruction::Nop),
                    // alternatively, return Err, but I could not for the life of me make that work
                    _ => Ok(Instruction::Invalid),
                }
            }
            // alternatively, return Err, but I could not for the life of me make that work
            _ => Ok(Instruction::Invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(Ok(Instruction::Acc(42)), "acc +42".parse());
        assert_eq!(Ok(Instruction::Acc(-42)), "acc -42".parse());
        assert_eq!(Ok(Instruction::Jmp(42)), "jmp +42".parse());
        assert_eq!(Ok(Instruction::Jmp(-42)), "jmp -42".parse());
        assert_eq!(Ok(Instruction::Nop), "nop +42".parse());
        assert_eq!(Ok(Instruction::Nop), "nop -42".parse());
        assert_eq!(Ok(Instruction::Invalid), "fart -42".parse());
    }
}
