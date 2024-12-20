use std::fs;
use std::{collections::HashSet, env};

use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    let mut width = 0;
    let mut height = 0;
    let mut walls = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| {
                    if col > width {
                        width = col;
                    }
                    if row > height {
                        height = row;
                    }
                    if c == 'S' {
                        start.x = col;
                        start.y = row;
                        None
                    } else if c == 'E' {
                        end.x = col;
                        end.y = row;
                        None
                    } else if c == '#' {
                        Some(Position { x: col, y: row })
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();
    height += 1;
    width += 1;
    let max: i32 = if width < 20 { 1 } else { 100 };
    let result = dijkstra(
        &start,
        |p| {
            let mut result = Vec::new();
            if p.x > 0 && !walls.contains(&Position { x: p.x - 1, y: p.y }) {
                result.push((Position { x: p.x - 1, y: p.y }, 1));
            }
            if p.y > 0 && !walls.contains(&Position { x: p.x, y: p.y - 1 }) {
                result.push((Position { x: p.x, y: p.y - 1 }, 1));
            }
            if p.x < width - 1 && !walls.contains(&Position { x: p.x + 1, y: p.y }) {
                result.push((Position { x: p.x + 1, y: p.y }, 1));
            }
            if p.y < height - 1 && !walls.contains(&Position { x: p.x, y: p.y + 1 }) {
                result.push((Position { x: p.x, y: p.y + 1 }, 1));
            }
            result
        },
        |p| *p == end,
    );
    if result.is_none() {
        println!("No path found");
        return;
    }
    let (_, cost) = result.unwrap();
    let mut count = 0;
    for wall in walls.clone().iter() {
        let mut neighbor_walls = 0;
        if wall.x > 0
            && walls.contains(&Position {
                x: wall.x - 1,
                y: wall.y,
            })
        {
            neighbor_walls += 1;
        }

        if walls.contains(&Position {
            x: wall.x + 1,
            y: wall.y,
        }) {
            neighbor_walls += 1;
        }

        if wall.y > 0
            && walls.contains(&Position {
                x: wall.x,
                y: wall.y - 1,
            })
        {
            neighbor_walls += 1;
        }
        if walls.contains(&Position {
            x: wall.x,
            y: wall.y + 1,
        }) {
            neighbor_walls += 1;
        }
        if neighbor_walls != 2 {
            continue;
        }
        walls.remove(wall);
        if let Some((_, new_cost)) = dijkstra(
            &start,
            |p| {
                let mut result = Vec::new();
                if p.x > 0 && !walls.contains(&Position { x: p.x - 1, y: p.y }) {
                    result.push((Position { x: p.x - 1, y: p.y }, 1));
                }
                if p.y > 0 && !walls.contains(&Position { x: p.x, y: p.y - 1 }) {
                    result.push((Position { x: p.x, y: p.y - 1 }, 1));
                }
                if p.x < width - 1 && !walls.contains(&Position { x: p.x + 1, y: p.y }) {
                    result.push((Position { x: p.x + 1, y: p.y }, 1));
                }
                if p.y < height - 1 && !walls.contains(&Position { x: p.x, y: p.y + 1 }) {
                    result.push((Position { x: p.x, y: p.y + 1 }, 1));
                }
                result
            },
            |p| *p == end,
        ) {
            if (cost - new_cost) >= max {
                count += 1;
            }
        } else {
            println!("{:?} No path found", wall);
        }
        walls.insert(wall.clone());
    }
    println!("{:?}", count);
}
