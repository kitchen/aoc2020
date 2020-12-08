use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let identifier = &args[1]; // this will panic if there's no argument which is fine for this
    println!("seat id: {}", binseat(identifier));
}

fn binseat(identifier: &str) -> u16 {
    let mut seat = 0;
    for bit in identifier.to_string().chars() {
        match bit {
            'F' | 'L' => seat <<= 1,
            'B' | 'R' => {
                seat <<= 1;
                seat += 1;
            }
            _ => unimplemented!(),
        }
    }

    seat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        assert_eq!(357, binseat("FBFBBFFRLR"));
        assert_eq!(567, binseat("BFFFBBFRRR"));
        assert_eq!(119, binseat("FFFBBBFRRR"));
        assert_eq!(820, binseat("BBFFBBFRLL"));
    }
}
