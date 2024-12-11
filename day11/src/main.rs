use std::env;
use std::fs;

fn rules(num: u64) -> Vec<u64> {
    let mut result = Vec::new();

    if num == 0 {
        result.push(1);
    } else if num.ilog10() % 2 == 1 {
        let digits = num.ilog10() + 1;
        result.push(num / 10_u64.pow(digits / 2));
        result.push(num % 10_u64.pow(digits / 2));
    } else {
        result.push(num * 2024)
    }
    result
}

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let blinks = 25u8;
    let mut numbers = fs::read_to_string(args[1].clone())
        .unwrap()
        .split_whitespace()
        .map(|x| (x.parse::<u64>().unwrap(), blinks))
        .collect::<Vec<(u64, u8)>>();

    let mut sum = 0;
    while let Some((num, blink)) = numbers.pop() {
        if blink == 0 {
            sum += 1;
            continue;
        }
        rules(num).iter().for_each(|x| {
            numbers.push((*x, blink - 1));
        });
    }

    println!("{:?}", sum);
}

#[cfg(feature = "second")]
fn main() {
    use std::collections::HashMap;

    let args: Vec<String> = env::args().collect();

    let mut numbers = fs::read_to_string(args[1].clone())
        .unwrap()
        .split_whitespace()
        .map(|x| (x.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();
    let mut next: HashMap<u64, u64> = HashMap::new();
    for _ in 0..75 {
        next.clear();
        for (num, count) in &numbers {
            rules(*num).iter().for_each(|x| {
                let new_count = next.entry(*x).or_insert(0);
                *new_count += count;
            });
        }
        numbers = next.clone();
    }
    println!(
        "{:?}",
        numbers.iter().fold(0, |acc, (_, count)| acc + count)
    );
}
