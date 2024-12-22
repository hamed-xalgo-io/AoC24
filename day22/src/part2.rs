use std::env;
use std::{collections::HashMap, fs};

use itertools::Itertools;

fn secret_number(seed: i64) -> i64 {
    let secret = (seed * 64) ^ seed;
    let secret = secret % 16777216;
    let secret = (secret / 32) ^ secret;
    let secret = secret % 16777216;
    let secret = (secret * 2048) ^ secret;
    secret % 16777216
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut instruction_map: HashMap<(i64, i64, i64, i64), HashMap<usize, i64>> = HashMap::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .for_each(|(buyer, line)| {
            let mut secret = line.parse::<i64>().unwrap();
            let price_changes = (0..2000)
                .map(|_| {
                    let result = secret;
                    secret = secret_number(secret);
                    result
                })
                .tuple_windows()
                .map(|(a, b)| (b % 10, b % 10 - a % 10))
                .collect::<Vec<(i64, i64)>>();
            for i in 0..price_changes.len() - 4 {
                let instr = price_changes[i..i + 5]
                    .iter()
                    .map(|(_a, b)| *b)
                    .collect::<Vec<_>>();
                let instr_buyers = instruction_map
                    .entry((instr[0], instr[1], instr[2], instr[3]))
                    .or_default();
                instr_buyers.entry(buyer).or_insert(price_changes[i + 3].0);
            }
        });
    let mut max_bananas = 0;
    let mut max_instr = (0, 0, 0, 0);
    for (_instr, buyers) in instruction_map {
        let mut buyer_max: HashMap<usize, i64> = HashMap::new();
        for (buyer, price) in buyers {
            let max = buyer_max.entry(buyer).or_default();
            *max = (*max).max(price);
        }
        let bananas = buyer_max.values().sum::<i64>();
        if bananas > max_bananas {
            max_bananas = bananas;
            max_instr = _instr;
        }
    }
    println!("{:?}", max_instr);
    println!("{}", max_bananas);
}
