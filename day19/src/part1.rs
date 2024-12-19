use std::fs;
use std::{collections::HashSet, env};

fn possible_design(towels: &HashSet<&str>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        if let Some(new_design) = design.strip_prefix(towel) {
            if possible_design(towels, new_design) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(args[1].clone()).expect("Something went wrong reading the file");
    let lines = data.lines().collect::<Vec<&str>>();
    let towels = lines[0].split(", ").collect::<HashSet<&str>>();
    let answer = lines[2..]
        .iter()
        .filter(|d| possible_design(&towels, d))
        .count();

    println!("{}", answer);
}
