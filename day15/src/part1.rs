use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn process_movement(
    mv: char,
    robot: &mut Position,
    boxes: &mut HashSet<Position>,
    walls: &HashSet<Position>,
) {
    match mv {
        '<' => {
            let new_pos = Position {
                x: robot.x - 1,
                y: robot.y,
            };
            if !walls.contains(&new_pos) && !boxes.contains(&new_pos) {
                println!("Move");
                robot.x -= 1;
                return;
            }
            if walls.contains(&new_pos) {
                println!("Hit a wall");
                return;
            }
            let mut d = 2;
            loop {
                let new_pos = Position {
                    x: robot.x - d,
                    y: robot.y,
                };
                if walls.contains(&new_pos) {
                    println!("Hit a wall");
                    return;
                }
                if !boxes.contains(&new_pos) {
                    println!("Push {} boxes", d);
                    robot.x -= 1;
                    boxes.remove(robot);
                    boxes.insert(new_pos);
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
            if !walls.contains(&new_pos) && !boxes.contains(&new_pos) {
                println!("Move");
                robot.y -= 1;
                return;
            }
            if walls.contains(&new_pos) {
                println!("Hit a wall");
                return;
            }
            let mut d = 2;
            loop {
                let new_pos = Position {
                    x: robot.x,
                    y: robot.y - d,
                };
                if walls.contains(&new_pos) {
                    println!("Hit a wall");
                    return;
                }
                if !boxes.contains(&new_pos) {
                    println!("Push {} boxes", d);
                    robot.y -= 1;
                    boxes.remove(robot);
                    boxes.insert(new_pos);
                    return;
                }
                d += 1;
            }
        }
        '>' => {
            let new_pos = Position {
                x: robot.x + 1,
                y: robot.y,
            };
            if !walls.contains(&new_pos) && !boxes.contains(&new_pos) {
                println!("Move");
                robot.x += 1;
                return;
            }
            if walls.contains(&new_pos) {
                println!("Hit a wall");
                return;
            }
            let mut d = 2;
            loop {
                let new_pos = Position {
                    x: robot.x + d,
                    y: robot.y,
                };
                if walls.contains(&new_pos) {
                    println!("Hit a wall");
                    return;
                }
                if !boxes.contains(&new_pos) {
                    println!("Push {} boxes", d);
                    robot.x += 1;
                    boxes.remove(robot);
                    boxes.insert(new_pos);
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
            if !walls.contains(&new_pos) && !boxes.contains(&new_pos) {
                println!("Move");
                robot.y += 1;
                return;
            }
            if walls.contains(&new_pos) {
                println!("Hit a wall");
                return;
            }
            let mut d = 2;
            loop {
                let new_pos = Position {
                    x: robot.x,
                    y: robot.y + d,
                };
                if walls.contains(&new_pos) {
                    println!("Hit a wall");
                    return;
                }
                if !boxes.contains(&new_pos) {
                    println!("Push {} boxes", d);
                    robot.y += 1;
                    boxes.remove(robot);
                    boxes.insert(new_pos);
                    return;
                }
                d += 1;
            }
        }
        _ => panic!("Invalid movement"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut boxes: HashSet<Position> = HashSet::new();
    let mut walls: HashSet<Position> = HashSet::new();
    let mut robot = Position { x: 0, y: 0 };
    let mut movements: String = String::new();
    let mut moves = false;
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
                    match c {
                        '#' => {
                            walls.insert(Position {
                                x: jdx as i32,
                                y: idx as i32,
                            });
                        }
                        'O' => {
                            boxes.insert(Position {
                                x: jdx as i32,
                                y: idx as i32,
                            });
                        }
                        '@' => {
                            robot = Position {
                                x: jdx as i32,
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
        process_movement(mv, &mut robot, &mut boxes, &walls);
    }

    let answer: i32 = boxes.iter().map(|b| 100 * b.y + b.x).sum();
    println!("{:?}", answer);
}
