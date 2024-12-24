use std::{collections::HashMap, fs, vec};
use std::{collections::HashSet, env};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut section = 0;
    let mut vars: HashMap<String, u64> = HashMap::new();
    let mut connections: HashMap<String, (String, String, String)> = HashMap::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                section += 1;
                return;
            }
            if section == 0 {
                let (a, b) = line.split_once(": ").unwrap();
                vars.insert(a.to_string(), b.parse().unwrap());
            } else {
                let (a, b) = line.split_once(" -> ").unwrap();
                let a = a.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                connections.insert(
                    b.to_string(),
                    (a[0].to_string(), a[1].to_string(), a[2].to_string()),
                );
            }
        });
    let mut connections_orig = connections.clone();
    let (x, y, mut z) = evaluate_all(vars.clone(), &connections).unwrap();
    let mut swapped: Vec<(String, String)> = Vec::new();
    let mut forbidden: HashSet<(String, String)> = HashSet::new();
    while x + y != z {
        let diff = z ^ (x + y);
        println!("Diff: {}", diff);
        println!(" X+Y: {:b}", x + y);
        println!("   Z: {:b}", z);
        println!("Diff: {:046b}", diff);
        println!("Diff: {}", diff.ilog2());
        let wrong_bit = diff.ilog2();
        let mut found = false;
        for perm in connections
            .clone()
            .keys()
            .filter(|k| !k.starts_with("x") && !k.starts_with("y"))
            .permutations(2)
        {
            let v1 = perm[0];
            let v2 = perm[1];
            if *v1 > *v2 || forbidden.contains(&(v1.clone(), v2.clone())) {
                continue;
            }

            connections = swap(v1, v2, connections);
            if let Ok((_, _, zi)) = evaluate_all(vars.clone(), &connections) {
                let diffi = zi ^ (x + y);
                if diffi == 0 || diffi.ilog2() < wrong_bit {
                    println!(
                        "Swapped {:?} {:?} {}",
                        v1,
                        v2,
                        if diffi > 0 { diffi.ilog2() } else { 0 }
                    );
                    swapped.push((v1.clone(), v2.clone()));
                    z = zi;
                    found = true;
                    break;
                } else {
                    // println!("Reverting 0 {:?} {:?}", v1, v2);
                    connections = swap(v1, v2, connections);
                }
            } else {
                // println!("Reverting 1 {:?} {:?}", v1, v2);
                connections = swap(v1, v2, connections);
            };
        }
        if !found || (z != x + y && swapped.len() >= 4) {
            if let Some((v1, v2)) = swapped.pop() {
                println!("Reverting 2 {:?} {:?} :{}", v1, v2, swapped.len() + 1);
                connections = swap(&v1, &v2, connections);
                let zi = evaluate_all(vars.clone(), &connections).unwrap().2;
                z = zi;
                forbidden.insert((v1, v2));
            } else {
                println!("No solution found");
                break;
            }
        }
    }
    println!("Swapped {:?}", swapped);
    for sw in swapped {
        connections_orig = swap(&sw.0, &sw.1, connections_orig);
    }
    let (x, y, z) = evaluate_all(vars.clone(), &connections).unwrap();
    println!("X+Y: {:b}", x + y);
    println!("Z: {:b}", z);
    println!("V: {:b}", x + y - z);
}

#[allow(dead_code)]
fn parents(i: usize, connections: &HashMap<String, (String, String, String)>) -> HashSet<String> {
    let mut queue = vec![format!("z{:02}", i)];
    let mut visited = HashSet::new();
    while let Some(v) = queue.pop() {
        if visited.contains(&v) {
            continue;
        }
        visited.insert(v.clone());
        let (l, _, r) = connections.get(&v).unwrap();
        if !l.starts_with("x") && !l.starts_with("y") {
            queue.push(l.clone());
        }
        if !r.starts_with("x") && !r.starts_with("y") {
            queue.push(r.clone());
        }
    }
    visited
}

#[allow(dead_code)]
fn next_bit(s1: &str, s2: &str) -> Option<usize> {
    for i in 0..s1.len() {
        let idx = s1.len() - i - 1;
        if s1.chars().nth(idx).unwrap() == s2.chars().nth(idx).unwrap() {
            continue;
        }
        return Some(i);
    }
    None
}

fn swap(
    v1: &str,
    v2: &str,
    mut connections: HashMap<String, (String, String, String)>,
) -> HashMap<String, (String, String, String)> {
    // print!("Swapping {} {} {}", v1, v2, connections.len());
    let n1 = connections.remove(v1).unwrap().clone();
    let n2 = connections.remove(v2).unwrap().clone();
    connections.insert(v1.to_string(), n2);
    connections.insert(v2.to_string(), n1);
    // println!(" -> {}", connections.len());
    connections
}

#[allow(dead_code)]
fn kishpil(
    i: usize,
    visited: &HashSet<String>,
    forbidden: &HashSet<(String, String)>,
    connections: &HashMap<String, (String, String, String)>,
    vars: &HashMap<String, u64>,
) -> Option<(String, String, usize)> {
    let (x, y, z) = evaluate_all(vars.clone(), connections).unwrap();
    let x_y = format!("{:b}", x + y);
    let z = format!("{:b}", z);
    let idx = x_y.len() - i - 1;
    // let mut selected: HashSet<(String, String)> = HashSet::new();
    let mut best = None;
    let mut max_score = z.len() + 1;
    println!("Trying to fix bit {} {}", i, z.chars().nth(idx).unwrap());
    for a in visited.iter().permutations(2) {
        let v1 = a[0];
        let v2 = a[1];
        if v1 > v2 {
            continue;
        }
        if forbidden.contains(&(v1.clone(), v2.clone())) {
            println!("Forbidden {:?}", (v1, v2));
            continue;
        }
        let pconnections = swap(v1, v2, connections.clone());

        if let Ok((_x2, _y2, z2)) = evaluate_all(vars.clone(), &pconnections) {
            let z2 = format!("{:b}", z2);
            if x_y.chars().nth(idx).unwrap() == z2.chars().nth(idx).unwrap() {
                let expected = format!(
                    "{}{}{}",
                    z.chars().take(idx).collect::<String>(),
                    z2.chars().nth(idx).unwrap(),
                    z.chars().skip(idx + 1).collect::<String>()
                );
                let score = expected
                    .chars()
                    .zip(z2.chars())
                    .map(|(a, b)| if a == b { 0 } else { 1 })
                    .sum::<usize>();
                if score < max_score {
                    max_score = score;

                    best = Some((v1.to_string(), v2.to_string(), score));
                }
            }
        }
    }
    best
}
fn evaluate_all(
    mut vars: HashMap<String, u64>,
    connections: &HashMap<String, (String, String, String)>,
) -> Result<(u64, u64, u64), String> {
    for zgate in connections.keys() {
        if !zgate.starts_with("z") {
            continue;
        }
        if vars.contains_key(zgate) {
            continue;
        }

        let res = evaluate(zgate, &mut vars, connections, 2000)?;
        vars.entry(zgate.to_string()).insert_entry(res);
    }
    Ok((number("x", &vars), number("y", &vars), number("z", &vars)))
}

fn number(s: &str, vars: &HashMap<String, u64>) -> u64 {
    let gates = vars.iter().filter(|(k, _)| k.starts_with(s)).count();
    let mut answer = "".to_string();
    for i in 0..gates {
        let zgate = format!("{}{:02}", s, i);
        let zval = vars.get(&zgate).unwrap();
        answer = format!("{}{}", zval, answer);
    }

    u64::from_str_radix(&answer, 2).unwrap()
}
fn evaluate(
    v: &String,
    vars: &mut HashMap<String, u64>,
    connections: &HashMap<String, (String, String, String)>,
    depth: u64,
) -> Result<u64, String> {
    if depth == 0 {
        // println!("Depth reached");
        return Err("Depth reached".to_string());
    }
    let (l, op, r) = connections.get(v).unwrap();
    if !vars.contains_key(l) {
        let res = evaluate(l, vars, connections, depth - 1)?;
        vars.entry(l.to_string()).insert_entry(res);
    }
    if !vars.contains_key(r) {
        let res = evaluate(r, vars, connections, depth - 1)?;
        vars.entry(r.to_string()).insert_entry(res);
    }
    Ok(match op.as_str() {
        "AND" => vars[l] & vars[r],
        "OR" => vars[l] | vars[r],
        "XOR" => vars[l] ^ vars[r],
        _ => panic!("Unknown operator"),
    })
}
