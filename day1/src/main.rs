use std::collections::BinaryHeap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            let numbers = line
                .split(" ")
                .filter_map(|v| {
                    if v.is_empty() {
                        None
                    } else {
                        v.parse::<i64>().ok()
                    }
                })
                .collect::<Vec<_>>();
            left.push(numbers[0]);
            right.push(numbers[1]);
        });

    let answer = left
        .into_sorted_vec()
        .iter()
        .zip(right.into_sorted_vec().iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());
    println!("{}", answer);
}
