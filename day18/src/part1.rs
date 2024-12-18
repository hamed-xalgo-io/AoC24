use pathfinding::prelude::dijkstra;
use std::fs;
use std::{collections::HashSet, env};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn successors(
        &self,
        corrupted: &HashSet<Position>,
        width: usize,
        height: usize,
    ) -> Vec<(Position, usize)> {
        let mut result = vec![];
        if self.x > 0 {
            let pos = Position {
                x: self.x - 1,
                y: self.y,
            };
            if !corrupted.contains(&pos) {
                result.push((pos, 1));
            }
        }
        if self.y > 0 {
            let pos = Position {
                x: self.x,
                y: self.y - 1,
            };
            if !corrupted.contains(&pos) {
                result.push((pos, 1));
            }
        }
        if self.x < width - 1 {
            let pos = Position {
                x: self.x + 1,
                y: self.y,
            };
            if !corrupted.contains(&pos) {
                result.push((pos, 1));
            }
        }
        if self.y < height - 1 {
            let pos = Position {
                x: self.x,
                y: self.y + 1,
            };
            if !corrupted.contains(&pos) {
                result.push((pos, 1));
            }
        }
        result
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 70, y: 70 };
    let max = 1024;
    let width = 71;
    let height = 71;
    let corrupted: Vec<Position> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            Position { x, y }
        })
        .collect();
    for len in max..corrupted.len() {
        let current_currupted = corrupted[0..len].iter().cloned().collect::<HashSet<_>>();

        let result = dijkstra(
            &start,
            |p| p.successors(&current_currupted, width, height),
            |p| p == &end,
        );
        if let Some((_, cost)) = result {
            if len == max {
                println!("Part1: {}", cost);
            }
        } else {
            println!("Part2: {:?}", corrupted[0..len].iter().last().unwrap());
            break;
        }
    }
}
