use std::{collections::HashSet, env, fs};

use itertools::Itertools;
use pathfinding::prelude::{dijkstra, dijkstra_all};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn successors(
        &self,
        walls: &HashSet<Position>,
        width: i32,
        height: i32,
    ) -> Vec<(Position, i32)> {
        let mut result = Vec::new();
        if self.x > 0
            && !walls.contains(&Position {
                x: self.x - 1,
                y: self.y,
            })
        {
            result.push((
                Position {
                    x: self.x - 1,
                    y: self.y,
                },
                1,
            ));
        }
        if self.y > 0
            && !walls.contains(&Position {
                x: self.x,
                y: self.y - 1,
            })
        {
            result.push((
                Position {
                    x: self.x,
                    y: self.y - 1,
                },
                1,
            ));
        }
        if self.x < width - 1
            && !walls.contains(&Position {
                x: self.x + 1,
                y: self.y,
            })
        {
            result.push((
                Position {
                    x: self.x + 1,
                    y: self.y,
                },
                1,
            ));
        }
        if self.y < height - 1
            && !walls.contains(&Position {
                x: self.x,
                y: self.y + 1,
            })
        {
            result.push((
                Position {
                    x: self.x,
                    y: self.y + 1,
                },
                1,
            ));
        }
        result
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut spaces: Vec<Position> = Vec::new();
    let walls = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| {
                    if col as i32 > width {
                        width = col as i32;
                    }
                    if row as i32 > height {
                        height = row as i32;
                    }
                    let pos = Position {
                        x: col as i32,
                        y: row as i32,
                    };
                    if c == 'S' {
                        start = pos.clone();
                        spaces.push(pos);
                        None
                    } else if c == 'E' {
                        end = pos.clone();
                        spaces.push(pos);
                        None
                    } else if c == '#' {
                        Some(pos)
                    } else {
                        spaces.push(pos);
                        None
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();
    height += 1;
    width += 1;
    let max: i32 = if width < 20 { 50 } else { 100 };
    println!("{:?}", max);
    let (_path, cost) = dijkstra(
        &start,
        |p| p.successors(&walls, width, height),
        |p| *p == end,
    )
    .unwrap();

    let mut from_start = dijkstra_all(&start, |p| p.successors(&walls, width, height));
    let mut to_end = dijkstra_all(&end, |p| p.successors(&walls, width, height));
    from_start.insert(start.clone(), (start.clone(), 0));
    to_end.insert(end.clone(), (end.clone(), 0));

    let mut count = 0;

    spaces.iter().combinations(2).for_each(|pair| {
        let p1 = pair[0];
        let p2 = pair[1];
        if p1.x == p2.x && p1.y == p2.y {
            return;
        }
        let dist = (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
        if !(2..=20).contains(&dist) {
            return;
        }

        let start_cost = from_start.get(p1).map(|(_, c)| *c).unwrap_or(10000);
        let end_cost = to_end.get(p2).map(|(_, c)| *c).unwrap_or(10000);
        let new_cost = start_cost + end_cost + dist;
        if (cost - new_cost) >= max {
            count += 1;
        }

        let start_cost = from_start.get(p2).map(|(_, c)| *c).unwrap_or(10000);
        let end_cost = to_end.get(p1).map(|(_, c)| *c).unwrap_or(10000);
        let new_cost = start_cost + end_cost + dist;
        if (cost - new_cost) >= max {
            count += 1;
        }
    });

    println!("{:?}", count);
}
