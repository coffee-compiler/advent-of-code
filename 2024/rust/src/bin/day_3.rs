use std::fs;

use regex::Regex;

fn main() {
    let data = fs::read_to_string("assets/day_3.txt").unwrap();

    let result_p1 = execute_instructions(&data);
    let result_p2 = filter_execute_instructions(&data);

    println!("{}", result_p1);
    println!("{}", result_p2);
}

fn execute_instructions(data: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for (_, [n1, n2]) in regex.captures_iter(data).map(|c| c.extract()) {
        total += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    }

    total
}

fn filter_execute_instructions(data: &str) -> i32 {
    let unnecessary_instructions_regex = Regex::new(r"don't\(\)(.|\n|\r)*?do\(\)").unwrap();
    let cleared_data = unnecessary_instructions_regex.replace_all(data, "");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;

    for (_, [n1, n2]) in regex.captures_iter(&cleared_data).map(|c| c.extract()) {
        total += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    }

    total
}
