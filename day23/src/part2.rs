use std::{collections::HashMap, fs};
use std::{collections::HashSet, env};

fn bors_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    connections: &HashMap<String, Vec<String>>,
    mut cliques: Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.into_iter().collect());
        return cliques;
    }
    let mut p = p.clone();
    let mut x = x.clone();
    for v in p.clone().iter().collect::<Vec<_>>() {
        cliques = bors_kerbosch(
            r.union(&HashSet::from([v.clone()])).cloned().collect(),
            p.intersection(&connections[v].iter().cloned().collect())
                .cloned()
                .collect(),
            x.intersection(&connections[v].iter().cloned().collect())
                .cloned()
                .collect(),
            connections,
            cliques,
        );
        p.remove(v);
        x.insert(v.clone());
    }
    cliques
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            let (a, b) = line.split_once("-").unwrap();
            connections
                .entry(a.to_string())
                .or_default()
                .push(b.to_string());
            connections
                .entry(b.to_string())
                .or_default()
                .push(a.to_string());
        });

    let nodes = connections.keys().cloned().collect::<HashSet<_>>();
    let cliques = bors_kerbosch(
        HashSet::new(),
        nodes,
        HashSet::new(),
        &connections,
        Vec::new(),
    );
    let mut max = 0;
    let mut max_scc = vec![];
    for scc in cliques.iter() {
        if scc.len() > max {
            max = scc.len();
            max_scc = scc.clone();
        }
    }
    max_scc.sort();
    println!("{:?}", max_scc.join(","));
}
