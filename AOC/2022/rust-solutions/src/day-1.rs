use std::fs::File;
use std::io::{BufRead, BufReader};


fn read_data() -> Vec<i64> {
    let file = File::open("input.txt").expect("File not found");
    let br = BufReader::new(file);
    let mut vect = Vec::new();

    for line in br.lines() {
        let line = line.unwrap();
        vect.push(line.parse::<i64>().unwrap());
    }
    vect
}


fn positive_changes(depth: &Vec<i64>) -> i64 {
    let mut count = 0;
    let mut previous = depth[0];

    for depth in depth.iter() {
        if previous < *depth {
            count += 1;
        }
        previous = *depth;
    }
    count
}


fn  sliding_window(depth: &Vec<i64>) -> i64 {
        let mut count: i64 = 0;
        let mut previous: Option<i64> = None;
        let mut curreent: i64;

        for i in depth.windows(3) {
            curreent =  i.iter().sum::<i64>();
            if let Some(p) = previous {
                if curreent > p {
                    count += 1;
                }
            }
            previous = Some(curreent);
        }
        count
}


fn main() {
    println!("[Total Increasing Measurements] {}", positive_changes(&read_data()));
    println!("[Sliding Window Measurements] {}", sliding_window(&read_data()));
}
