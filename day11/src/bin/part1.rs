use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, stdin, BufRead, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // this will panic if there's no argument which is fine for this
    let file = File::open(filename).expect("couldn't open file");

    let mut seat_map: SeatMap = Default::default();
    for (y, line) in io::BufReader::new(file).lines().enumerate() {
        let y = y as i64;
        let line = line.unwrap();
        for (x, seat) in line.chars().enumerate() {
            let x = x as i64;
            let seat = match seat {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                _ => panic!("invalid seat type at {}, {}: {}", x, y, seat),
            };
            seat_map.map.insert((x, y), seat);
            seat_map.max_x = x;
        }
        seat_map.max_y = y;
    }

    println!("{}", seat_map);
    let mut rounds = 0;
    while seat_map.apply_rules() {
        rounds += 1;
        println!("round: {}", rounds);
        println!("{}", seat_map);
        // stdin().read(&mut [0]).unwrap();
    }
    println!("seat map stopped changing after {} rounds", rounds);
    println!("occupied seats: {}", seat_map.num_occupied_seats());
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Seat {
    Floor,
    Occupied,
    Empty,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chr = match self {
            Seat::Floor => '.',
            Seat::Occupied => '#',
            Seat::Empty => 'L',
        };
        write!(f, "{}", chr)
    }
}

#[derive(Debug, Default)]
struct SeatMap {
    map: HashMap<(i64, i64), Seat>,
    max_x: i64,
    max_y: i64,
}

impl fmt::Display for SeatMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = "".to_string();
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                output.push_str(&self.map.get(&(x, y)).unwrap().to_string());
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl SeatMap {
    fn num_occupied_seats(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, &seat)| seat == Seat::Occupied)
            .count()
    }

    fn apply_rules(&mut self) -> bool {
        let mut new_map: HashMap<(i64, i64), Seat> = HashMap::new();
        let mut changed = false;
        for ((x, y), seat) in self.map.iter() {
            let x = *x;
            let y = *y;
            if seat == &Seat::Floor {
                new_map.insert((x, y), Seat::Floor);
                continue;
            }
            let mut occupied_neighbors = 0;
            for check_x in (x - 1)..=(x + 1) {
                for check_y in (y - 1)..=(y + 1) {
                    if x == check_x && y == check_y {
                        continue;
                    }
                    match self.map.get(&(check_x, check_y)) {
                        Some(Seat::Occupied) => occupied_neighbors += 1,
                        _ => {}
                    }
                }
            }
            // println!(
            //     "seat: ({}, {}) -> {}, occupied_neighbors: {}",
            //     x, y, seat, occupied_neighbors
            // );
            let new_seat = match (seat, occupied_neighbors) {
                (Seat::Occupied, _) if occupied_neighbors >= 4 => {
                    changed = true;
                    // println!("seat at ({}, {}) -> Empty", x, y);
                    Seat::Empty
                }
                (Seat::Empty, _) if occupied_neighbors == 0 => {
                    changed = true;
                    // println!("seat at ({}, {}) -> Occupied", x, y);
                    Seat::Occupied
                }
                _ => *seat,
            };
            new_map.insert((x, y), new_seat);
        }
        self.map = new_map;
        changed
    }
}
