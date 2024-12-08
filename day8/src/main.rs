use std::mem::replace;
use std::{collections::HashMap, env};
use std::{collections::HashSet, fs};

use itertools::Itertools;

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut anthenas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut max_loc = (0i32, 0i32);
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .for_each(|(r, line)| {
            if r as i32 > max_loc.0 {
                max_loc.0 = r as i32;
            }
            line.chars().enumerate().for_each(|(c, a)| {
                if c as i32 > max_loc.1 {
                    max_loc.1 = c as i32;
                }
                if a == '.' {
                    return;
                }

                anthenas
                    .entry(a)
                    .and_modify(|e| e.push((r as i32, c as i32)))
                    .or_insert(vec![(r as i32, c as i32)]);
            })
        });
    println!("{:?}", max_loc);
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    anthenas.values().for_each(|locations| {
        locations.iter().combinations(2).for_each(|pair| {
            let mut a1 = pair[0];
            let mut a2 = pair[1];
            if a1.0 > a2.0 {
                a1 = replace(&mut a2, a1)
            }
            let dr = (a1.0 - a2.0).abs();
            let dc = (a1.1 - a2.1).abs();
            let (t, d) = if a1.1 < a2.1 {
                ((a1.0 - dr, a1.1 - dc), (a2.0 + dr, a2.1 + dc))
            } else {
                ((a1.0 - dr, a1.1 + dc), (a2.0 + dr, a2.1 - dc))
            };
            if t.0 >= 0 && t.1 >= 0 && t.0 <= max_loc.0 && t.1 <= max_loc.1 {
                antinodes.insert(t);
            }
            if d.0 >= 0 && d.1 >= 0 && d.0 <= max_loc.0 && d.1 <= max_loc.1 {
                antinodes.insert(d);
            }
        });
    });

    println!("{:?}", antinodes.len());
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut anthenas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut max_loc = (0i32, 0i32);
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .for_each(|(r, line)| {
            if r as i32 > max_loc.0 {
                max_loc.0 = r as i32;
            }
            line.chars().enumerate().for_each(|(c, a)| {
                if c as i32 > max_loc.1 {
                    max_loc.1 = c as i32;
                }
                if a == '.' {
                    return;
                }

                anthenas
                    .entry(a)
                    .and_modify(|e| e.push((r as i32, c as i32)))
                    .or_insert(vec![(r as i32, c as i32)]);
            })
        });

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    anthenas.values().for_each(|locations| {
        locations.iter().combinations(2).for_each(|pair| {
            let mut a1 = pair[0];
            let mut a2 = pair[1];
            antinodes.insert(*a1);
            antinodes.insert(*a2);
            if a1.0 > a2.0 {
                a1 = replace(&mut a2, a1)
            }
            let dr = (a1.0 - a2.0).abs();
            let dc = (a1.1 - a2.1).abs();
            if a1.1 < a2.1 {
                let mut t = (a1.0 - dr, a1.1 - dc);
                while t.0 >= 0 && t.1 >= 0 && t.0 <= max_loc.0 && t.1 <= max_loc.1 {
                    antinodes.insert(t);
                    t = (t.0 - dr, t.1 - dc);
                }
                let mut d = (a2.0 + dr, a2.1 + dc);
                while d.0 >= 0 && d.1 >= 0 && d.0 <= max_loc.0 && d.1 <= max_loc.1 {
                    antinodes.insert(d);
                    d = (d.0 + dr, d.1 + dc);
                }
            } else {
                let mut t = (a1.0 - dr, a1.1 + dc);
                while t.0 >= 0 && t.1 >= 0 && t.0 <= max_loc.0 && t.1 <= max_loc.1 {
                    antinodes.insert(t);
                    t = (t.0 - dr, t.1 + dc);
                }
                let mut d = (a2.0 + dr, a2.1 - dc);
                while d.0 >= 0 && d.1 >= 0 && d.0 <= max_loc.0 && d.1 <= max_loc.1 {
                    antinodes.insert(d);
                    d = (d.0 + dr, d.1 - dc);
                }
            }
        });
    });

    println!("{:?}", antinodes.len());
}
