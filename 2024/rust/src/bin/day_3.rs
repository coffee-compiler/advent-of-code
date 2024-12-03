use std::fs;

use regex::Regex;

fn main() {
    let data = fs::read_to_string("assets/day_3.txt").unwrap();

    let result_p1 = puzzle1(&data);

    println!("{}", result_p1);
}

fn puzzle1(data: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for (_, [n1, n2]) in regex.captures_iter(data).map(|c| c.extract()) {
        total += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    }

    total
}