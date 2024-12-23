use std::fs;
use std::{collections::HashSet, env};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut unique: HashSet<String> = HashSet::new();
    let mut connections: HashSet<String> = HashSet::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            let (a, b) = line.split_once("-").unwrap();
            if a.starts_with("t") || b.starts_with("t") {
                unique.insert(a.to_string());
                unique.insert(b.to_string());
            }
            connections.insert(format!("{}-{}", a, b));
            connections.insert(format!("{}-{}", b, a));
        });
    let mut answer = 0;
    unique.iter().permutations(3).for_each(|perm| {
        let a = perm[0];
        let b = perm[1];
        let c = perm[2];
        if connections.contains(&format!("{}-{}", a, b))
            && connections.contains(&format!("{}-{}", b, c))
            && connections.contains(&format!("{}-{}", c, a))
            && a < b
            && b < c
            && (a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        {
            answer += 1;
        }
    });
    println!("{}", answer);
}
