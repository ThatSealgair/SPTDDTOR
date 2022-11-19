use itertools::Itertools;
use Command::{Down, Forward, Up};


const DATA: &str = include_str!("input.txt");


enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}


impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let (direction, amount) = s.split_whitespace().collect_tuple().unwrap();
        let n = amount.parse().unwrap();

        match direction {
            "forward" => Forward(n),
            "down" => Down(n),
            "up" => Up(n),
            _ => panic!(),
        }
    }
}


fn navigation(data: &str) -> i64 {
    let mut dist: i64 = 0;
    let mut depth: i64 = 0;

    for command in data.lines().map(Command::from) {
        match command {
            Forward(n) => dist += n,
            Down(n) => depth += n,
            Up(n) => depth -= n,
        }
    }
    dist * depth
}


fn aim(data: &str) -> i64 {
    let mut dist:i64 = 0;
    let mut depth:i64 = 0;
    let mut aim:i64 = 0;
    
    for command in data.lines().map(Command::from) {
        match command {
            Forward(n) => {
                dist += n;
                depth += aim * n
            }
            Down(n) => aim += n,
            Up(n) => aim -= n,
        }
    }
    dist * depth
}


fn main() {
    println!("[Distance Travelled] {}", navigation(DATA));
    println!("[Aim Offset] {}", aim(DATA));
}
