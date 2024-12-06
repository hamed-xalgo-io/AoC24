use std::fs;
use std::{collections::HashSet, env};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_guard(grid: &[Vec<char>]) -> (usize, usize) {
    let mut pos = (0, 0);
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, col)| {
            if *col == '^' {
                pos = (i, j);
            }
        });
    });
    pos
}

fn mark_guard(grid: &mut [Vec<char>], i: usize, j: usize) -> Option<(isize, isize)> {
    let mut dir = Direction::Up;
    let mut i = i as isize;
    let mut j = j as isize;
    let mut path: HashSet<(isize, isize, Direction)> = HashSet::new();
    while i > 0 && j > 0 && i < grid.len() as isize && j < grid[i as usize].len() as isize {
        path.insert((i, j, dir.clone()));
        grid[i as usize][j as usize] = 'X';
        match dir {
            Direction::Up => {
                if i > 0
                    && (grid[i as usize - 1][j as usize] == '.'
                        || grid[i as usize - 1][j as usize] == 'X')
                {
                    grid[i as usize - 1][j as usize] = 'X';
                    i -= 1;
                } else if i > 0 {
                    dir = Direction::Right;
                } else {
                    break;
                }
            }
            Direction::Down => {
                if i < grid.len() as isize - 1
                    && (grid[i as usize + 1][j as usize] == '.'
                        || grid[i as usize + 1][j as usize] == 'X')
                {
                    grid[i as usize + 1][j as usize] = 'X';
                    i += 1;
                } else if i < grid.len() as isize - 1 {
                    dir = Direction::Left;
                } else {
                    break;
                }
            }
            Direction::Left => {
                if j > 0
                    && (grid[i as usize][j as usize - 1] == '.'
                        || grid[i as usize][j as usize - 1] == 'X')
                {
                    grid[i as usize][j as usize - 1] = 'X';
                    j -= 1;
                } else if j > 0 {
                    dir = Direction::Up;
                } else {
                    break;
                }
            }
            Direction::Right => {
                if j < grid[i as usize].len() as isize - 1
                    && (grid[i as usize][j as usize + 1] == '.'
                        || grid[i as usize][j as usize + 1] == 'X')
                {
                    grid[i as usize][j as usize + 1] = 'X';
                    j += 1;
                } else if j < grid[i as usize].len() as isize - 1 {
                    dir = Direction::Down;
                } else {
                    break;
                }
            }
        }
        if path.contains(&(i, j, dir.clone())) {
            return None;
        }
    }
    Some((i, j))
}

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut grid: Vec<Vec<char>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut answer: u32 = 0;

    let (gi, gj) = find_guard(&grid);
    mark_guard(&mut grid, gi, gj);
    grid.iter().for_each(|row| {
        row.iter().for_each(|col| {
            if *col == 'X' {
                answer += 1;
            }
        });
    });
    println!("{:?}", answer);
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let grid: Vec<Vec<char>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut answer: u32 = 0;

    let (gi, gj) = find_guard(&grid);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if (i, j) != (gi, gj) {
                let mut new_grid = grid.clone();
                new_grid[i][j] = 'O';
                if mark_guard(&mut new_grid, gi, gj).is_none() {
                    answer += 1;
                }
            }
        }
    }
    println!("{:?}", answer);
}
