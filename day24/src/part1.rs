use std::env;
use std::{collections::HashMap, fs};

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
    for zgate in connections.keys() {
        if !zgate.starts_with("z") {
            continue;
        }
        if vars.contains_key(zgate) {
            continue;
        }

        let res = evaluate(zgate, &mut vars, &connections);
        vars.entry(zgate.to_string()).insert_entry(res);
    }
    let zgates = vars.iter().filter(|(k, _)| k.starts_with("z")).count();
    let mut answer = "".to_string();
    for i in 0..zgates {
        let zgate = format!("z{:02}", i);
        let zval = vars.get(&zgate).unwrap();
        answer = format!("{}{}", zval, answer);
    }
    println!("{:?}", u64::from_str_radix(&answer, 2));
}

fn evaluate(
    v: &String,
    vars: &mut HashMap<String, u64>,
    connections: &HashMap<String, (String, String, String)>,
) -> u64 {
    let (l, op, r) = connections.get(v).unwrap();
    if !vars.contains_key(l) {
        let res = evaluate(l, vars, connections);
        vars.entry(l.to_string()).insert_entry(res);
    }
    if !vars.contains_key(r) {
        let res = evaluate(r, vars, connections);
        vars.entry(r.to_string()).insert_entry(res);
    }
    match op.as_str() {
        "AND" => vars[l] & vars[r],
        "OR" => vars[l] | vars[r],
        "XOR" => vars[l] ^ vars[r],
        _ => panic!("Unknown operator"),
    }
}
