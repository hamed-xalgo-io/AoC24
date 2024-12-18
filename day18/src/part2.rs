use pathfinding::prelude::astar_bag;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Robot {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Robot {
    fn successors(&self, text: &[Vec<char>]) -> Vec<(Robot, usize)> {
        let height = text.len();
        let width = text[0].len();
        let mut result = vec![];
        match self.direction {
            Direction::Right => {
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Up,
                    },
                    1000,
                ));
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Down,
                    },
                    1000,
                ));
                if self.x + 1 < width && text[self.y][self.x + 1] != '#' {
                    result.push((
                        Robot {
                            x: self.x + 1,
                            y: self.y,
                            direction: Direction::Right,
                        },
                        1,
                    ));
                }
            }
            Direction::Left => {
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Up,
                    },
                    1000,
                ));
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Down,
                    },
                    1000,
                ));
                if self.x > 0 && text[self.y][self.x - 1] != '#' {
                    result.push((
                        Robot {
                            x: self.x - 1,
                            y: self.y,
                            direction: Direction::Left,
                        },
                        1,
                    ));
                }
            }
            Direction::Up => {
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Left,
                    },
                    1000,
                ));
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Right,
                    },
                    1000,
                ));
                if self.y > 0 && text[self.y - 1][self.x] != '#' {
                    result.push((
                        Robot {
                            x: self.x,
                            y: self.y - 1,
                            direction: Direction::Up,
                        },
                        1,
                    ));
                }
            }
            Direction::Down => {
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Left,
                    },
                    1000,
                ));
                result.push((
                    Robot {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Right,
                    },
                    1000,
                ));
                if self.y + 1 < height && text[self.y + 1][self.x] != '#' {
                    result.push((
                        Robot {
                            x: self.x,
                            y: self.y + 1,
                            direction: Direction::Down,
                        },
                        1,
                    ));
                }
            }
        }
        result
    }
    fn heuristic(&self, goal: &Robot) -> usize {
        let dx = if self.x > goal.x {
            self.x - goal.x
        } else {
            goal.x - self.x
        };
        let dy = if self.y > goal.y {
            self.y - goal.y
        } else {
            goal.y - self.y
        };
        if dx == 0 || dy == 0 {
            dx + dy
        } else {
            dx + dy + 1000
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut robot = Robot {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };
    let mut end = Robot {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };
    let text: Vec<Vec<char>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        robot.x = col;
                        robot.y = row;
                        '.'
                    } else if c == 'E' {
                        end.x = col;
                        end.y = row;
                        'E'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();
    let result = astar_bag(
        &robot,
        |p| p.successors(&text),
        |p| p.heuristic(&end),
        |p| text[p.y][p.x] == 'E',
    );
    let (all_paths, cost) = result.unwrap();

    let distinct_points: HashSet<(usize, usize)> = all_paths
        .into_iter()
        .flat_map(|path| path.into_iter().map(|r| (r.x, r.y)))
        .collect();

    println!("{:?}", distinct_points.len());
    println!("{:?}", cost);
}
