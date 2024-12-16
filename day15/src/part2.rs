use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
enum Side {
    Left,
    Right,
}

fn process_movement(
    mv: char,
    robot: &mut Position,
    boxes: &mut HashMap<Position, Side>,
    walls: &HashMap<Position, Side>,
) {
    match mv {
        '<' => {
            let new_pos = Position {
                x: robot.x - 1,
                y: robot.y,
            };

            if !walls.contains_key(&new_pos) && !boxes.contains_key(&new_pos) {
                println!("Move");
                robot.x -= 1;
                return;
            }
            if walls.contains_key(&new_pos) {
                println!("Hit a wall");
                return;
            }
            let mut d = 2;
            loop {
                let new_pos = Position {
                    x: robot.x - d,
                    y: robot.y,
                };
                if walls.contains_key(&new_pos) {
                    println!("Hit a wall");
                    return;
                }
                if !boxes.contains_key(&new_pos) {
                    println!("Push {} boxes", d);
                    for i in 1..d {
                        let pos = Position {
                            x: robot.x - d + i,
                            y: robot.y,
                        };
                        let side = boxes.remove(&pos).unwrap();
                        let pos = Position {
                            x: pos.x - 1,
                            y: pos.y,
                        };
                        boxes.insert(pos, side);
                    }
                    robot.x -= 1;
                    return;
                }
                d += 1;
            }
        }
        '^' => {
            let new_pos = Position {
                x: robot.x,
                y: robot.y - 1,
            };
            if !walls.contains_key(&new_pos) && !boxes.contains_key(&new_pos) {
                println!("Move");
                robot.y -= 1;
                return;
            }
            if walls.contains_key(&new_pos) {
                println!("Hit a wall");
                return;
            }

            let side = boxes.get(&new_pos).unwrap();
            let mut next_boxes: Vec<Position> = Vec::new();
            let mut all_movable_boxes: HashSet<Position> = HashSet::new();
            match side {
                Side::Left => {
                    next_boxes.push(Position {
                        x: new_pos.x,
                        y: new_pos.y,
                    });
                    next_boxes.push(Position {
                        x: new_pos.x + 1,
                        y: new_pos.y,
                    });
                }
                Side::Right => {
                    next_boxes.push(Position {
                        x: new_pos.x,
                        y: new_pos.y,
                    });
                    next_boxes.push(Position {
                        x: new_pos.x - 1,
                        y: new_pos.y,
                    });
                }
            }
            all_movable_boxes.extend(next_boxes.clone());

            loop {
                let next_boxes_copy = next_boxes.clone();
                next_boxes.clear();
                for b in next_boxes_copy.iter() {
                    let new_pos = Position { x: b.x, y: b.y - 1 };
                    if walls.contains_key(&new_pos) {
                        println!("Hit a wall");
                        return;
                    }
                    if boxes.contains_key(&new_pos) {
                        match boxes.get(&new_pos).unwrap() {
                            Side::Left => {
                                next_boxes.push(Position { x: b.x, y: b.y - 1 });
                                next_boxes.push(Position {
                                    x: b.x + 1,
                                    y: b.y - 1,
                                });
                            }
                            Side::Right => {
                                next_boxes.push(Position { x: b.x, y: b.y - 1 });
                                next_boxes.push(Position {
                                    x: b.x - 1,
                                    y: b.y - 1,
                                });
                            }
                        };
                    }
                }
                if next_boxes.is_empty() {
                    println!("Push {:?} boxes", all_movable_boxes.len());
                    robot.y -= 1;
                    let mut all_movable_boxes: Vec<_> = all_movable_boxes.iter().collect();
                    all_movable_boxes.sort_by(|a, b| a.y.cmp(&b.y));
                    for box_pos in all_movable_boxes.into_iter() {
                        let side = boxes.remove(box_pos).unwrap();
                        boxes.insert(
                            Position {
                                x: box_pos.x,
                                y: box_pos.y - 1,
                            },
                            side,
                        );
                    }
                    return;
                } else {
                    all_movable_boxes.extend(next_boxes.clone());
                }
            }
        }
        '>' => {
            let new_pos = Position {
                x: robot.x + 1,
                y: robot.y,
            };
            if !walls.contains_key(&new_pos) && !boxes.contains_key(&new_pos) {
                println!("Move");
                robot.x += 1;
                return;
            }
            if walls.contains_key(&new_pos) {
                println!("Hit a wall");
                return;
            }
            let mut d = 2;
            loop {
                let new_pos = Position {
                    x: robot.x + d,
                    y: robot.y,
                };
                if walls.contains_key(&new_pos) {
                    println!("Hit a wall");
                    return;
                }
                if !boxes.contains_key(&new_pos) {
                    println!("Push {} boxes", d);
                    for i in 1..d {
                        let mut pos = Position {
                            x: robot.x + d - i,
                            y: robot.y,
                        };
                        let side = boxes.remove(&pos).unwrap();
                        pos.x += 1;
                        boxes.insert(pos, side);
                    }
                    robot.x += 1;
                    return;
                }
                d += 1;
            }
        }
        'v' => {
            let new_pos = Position {
                x: robot.x,
                y: robot.y + 1,
            };
            if !walls.contains_key(&new_pos) && !boxes.contains_key(&new_pos) {
                println!("Move");
                robot.y += 1;
                return;
            }
            if walls.contains_key(&new_pos) {
                println!("Hit a wall");
                return;
            }

            let side = boxes.get(&new_pos).unwrap();
            let mut next_boxes: Vec<Position> = Vec::new();
            let mut all_movable_boxes: HashSet<Position> = HashSet::new();
            match side {
                Side::Left => {
                    next_boxes.push(Position {
                        x: new_pos.x,
                        y: new_pos.y,
                    });
                    next_boxes.push(Position {
                        x: new_pos.x + 1,
                        y: new_pos.y,
                    });
                }
                Side::Right => {
                    next_boxes.push(Position {
                        x: new_pos.x,
                        y: new_pos.y,
                    });
                    next_boxes.push(Position {
                        x: new_pos.x - 1,
                        y: new_pos.y,
                    });
                }
            }
            all_movable_boxes.extend(next_boxes.clone());

            loop {
                let next_boxes_copy = next_boxes.clone();
                next_boxes.clear();
                for b in next_boxes_copy.iter() {
                    let new_pos = Position { x: b.x, y: b.y + 1 };
                    if walls.contains_key(&new_pos) {
                        println!("Hit a wall");
                        return;
                    }
                    if boxes.contains_key(&new_pos) {
                        match boxes.get(&new_pos).unwrap() {
                            Side::Left => {
                                next_boxes.push(Position { x: b.x, y: b.y + 1 });
                                next_boxes.push(Position {
                                    x: b.x + 1,
                                    y: b.y + 1,
                                });
                            }
                            Side::Right => {
                                next_boxes.push(Position { x: b.x, y: b.y + 1 });
                                next_boxes.push(Position {
                                    x: b.x - 1,
                                    y: b.y + 1,
                                });
                            }
                        };
                    }
                }
                if next_boxes.is_empty() {
                    println!("Push {:?} boxes", all_movable_boxes.len());
                    robot.y += 1;
                    let mut all_movable_boxes: Vec<_> = all_movable_boxes.iter().collect();
                    all_movable_boxes.sort_by(|a, b| b.y.cmp(&a.y));
                    for box_pos in all_movable_boxes.into_iter() {
                        let side = boxes.remove(box_pos).unwrap();

                        boxes.insert(
                            Position {
                                x: box_pos.x,
                                y: box_pos.y + 1,
                            },
                            side,
                        );
                    }
                    return;
                } else {
                    all_movable_boxes.extend(next_boxes.clone());
                }
            }
        }
        _ => panic!("Invalid movement"),
    }
}

fn main() {
    use core::panic;

    let args: Vec<String> = env::args().collect();
    let mut boxes: HashMap<Position, Side> = HashMap::new();
    let mut walls: HashMap<Position, Side> = HashMap::new();
    let mut robot = Position { x: 0, y: 0 };
    let mut movements: String = String::new();
    let mut moves = false;
    let mut width = 0;
    let mut height = 0;
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            if line.is_empty() {
                moves = true;
                return;
            }
            line.chars().enumerate().for_each(|(jdx, c)| {
                if !moves {
                    if idx > height {
                        height = idx;
                    }
                    if jdx > width {
                        width = jdx;
                    }
                    match c {
                        '#' => {
                            walls.insert(
                                Position {
                                    x: jdx as i32 * 2,
                                    y: idx as i32,
                                },
                                Side::Left,
                            );
                            walls.insert(
                                Position {
                                    x: jdx as i32 * 2 + 1,
                                    y: idx as i32,
                                },
                                Side::Right,
                            );
                        }
                        'O' => {
                            boxes.insert(
                                Position {
                                    x: jdx as i32 * 2,
                                    y: idx as i32,
                                },
                                Side::Left,
                            );
                            boxes.insert(
                                Position {
                                    x: jdx as i32 * 2 + 1,
                                    y: idx as i32,
                                },
                                Side::Right,
                            );
                        }
                        '@' => {
                            robot = Position {
                                x: jdx as i32 * 2,
                                y: idx as i32,
                            };
                        }
                        _ => (),
                    }
                } else {
                    movements.push(c);
                }
            });
        });

    for mv in movements.chars() {
        let box_count = boxes.len();
        process_movement(mv, &mut robot, &mut boxes, &walls);
        if box_count != boxes.len() {
            panic!("Boxes count changed");
        }
    }

    let answer: i32 = boxes
        .iter()
        .map(|(p, s)| match s {
            Side::Left => 100 * p.y + p.x,
            Side::Right => 0,
        })
        .sum();
    println!("{:?}", answer);
}
