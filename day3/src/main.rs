use std::env;
use std::fs;

fn process_line(line: &str) -> i64 {
    line.split("mul(")
        .map(|mul| {
            if let Some(args) = mul.split_once(")") {
                if let Some((a, b)) = args.0.split_once(",") {
                    if let Ok(a) = a.parse::<i64>() {
                        if let Ok(b) = b.parse::<i64>() {
                            return a * b;
                        }
                    }
                }
            }
            0
        })
        .sum::<i64>()
}

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let answer: i64 = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| process_line(line))
        .sum();
    println!("{}", answer);
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let line = fs::read_to_string(args[1].clone()).expect("Something went wrong reading the file");

    let mut dos: Vec<String> = Vec::new();
    line.split("don't()").enumerate().for_each(|(i, dont)| {
        if i == 0 {
            dos.push(dont.to_string());
        } else if let Some((_a, b)) = dont.split_once("do()") {
            dos.push(b.to_string());
        }
    });
    let answer = dos.iter().map(|line| process_line(line)).sum::<i64>();

    println!("{}", answer);
}
