use std::collections::HashMap;
use std::env;
use std::fs;

fn need_fix(update: &[u8], rules: &HashMap<u8, Vec<u8>>) -> Option<(usize, usize)> {
    let mut visited: Vec<bool> = vec![false; 256];
    for (idx, item) in update.iter().enumerate() {
        visited[*item as usize] = true;
        if let Some(rule) = rules.get(item) {
            for pre in rule {
                if update.contains(pre) && !visited[*pre as usize] {
                    let i = update.iter().position(|v| v == pre).unwrap();
                    let j = idx;
                    return Some((i, j));
                }
            }
        }
    }
    None
}

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut updates: Vec<Vec<u8>> = Vec::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            if let Some((a, b)) = line.split_once('|') {
                let a = a.trim().parse::<u8>().unwrap();
                let b = b.trim().parse::<u8>().unwrap();
                rules.entry(b).or_default().push(a);

                rules.get_mut(&b).unwrap().push(a);
            }
            if line.contains(",") {
                updates.push(line.split(',').map(|x| x.trim().parse().unwrap()).collect());
            }
        });
    let mut answer: u32 = 0;
    for update in updates {
        if need_fix(&update, &rules).is_none() {
            answer += update[update.len() / 2] as u32;
        }
    }
    println!("{:?}", answer);
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut updates: Vec<Vec<u8>> = Vec::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            if let Some((a, b)) = line.split_once('|') {
                let a = a.trim().parse::<u8>().unwrap();
                let b = b.trim().parse::<u8>().unwrap();
                rules.entry(b).or_default().push(a);
            }
            if line.contains(",") {
                updates.push(line.split(',').map(|x| x.trim().parse().unwrap()).collect());
            }
        });
    let mut answer: u32 = 0;
    for update in updates.iter_mut() {
        let mut correct = true;
        while let Some((i, j)) = need_fix(update, &rules) {
            let item = update.remove(i);
            update.insert(j, item);
            correct = false;
        }
        if !correct {
            answer += update[update.len() / 2] as u32;
        }
    }
    println!("{:?}", answer);
}
