use std::env;
use std::{collections::HashMap, fs};

fn possible_design(towels: &Vec<&str>, design: &str, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = cache.get(design) {
        return count;
    }
    if design.is_empty() {
        return 1;
    }
    let mut count = 0;
    for towel in towels {
        if let Some(new_design) = design.strip_prefix(towel) {
            count += possible_design(towels, new_design, cache);
        }
    }
    cache.insert(design.to_string(), count);
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(args[1].clone()).expect("Something went wrong reading the file");
    let lines = data.lines().collect::<Vec<&str>>();
    let towels = lines[0].split(", ").collect::<Vec<&str>>();
    let mut cache: HashMap<String, usize> = HashMap::new();
    let answer = lines[2..]
        .iter()
        .map(|d| possible_design(&towels, d, &mut cache))
        .sum::<usize>();

    println!("{}", answer);
}
