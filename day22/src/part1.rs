use std::env;
use std::fs;

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
    let answer = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let mut secret = line.parse::<i64>().unwrap();
            for _ in 0..2000 {
                secret = secret_number(secret);
            }
            secret
        })
        .sum::<i64>();
    println!("{}", answer);
}
