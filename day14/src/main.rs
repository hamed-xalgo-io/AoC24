use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut robots: Vec<Robot> = Vec::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let (x, y) = p.split_once(",").unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            let (_, v) = v.split_once("=").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();
            let vx = vx.parse::<i32>().unwrap();
            let vy = vy.parse::<i32>().unwrap();
            robots.push(Robot { x, y, vx, vy });
        });

    let width = 101;
    let height = 103;
    let mut grid = vec![vec![0; 101]; 103];
    let mut blocks: Vec<u32> = vec![0; 4];
    for robot in robots.iter() {
        let mut x = (robot.x + robot.vx * 100) % width;
        let mut y = (robot.y + robot.vy * 100) % height;
        if x < 0 {
            x += width;
        }
        if y < 0 {
            y += height;
        }
        println!("{}, {}", x, y);
        grid[y as usize][x as usize] += 1;
        if 2 * x + 1 == width || 2 * y + 1 == height {
            continue;
        }
        if 2 * x < width && 2 * y < height {
            blocks[0] += 1;
        } else if 2 * x > width && 2 * y < height {
            blocks[1] += 1;
        } else if 2 * x < width && 2 * y > height {
            blocks[2] += 1;
        } else if 2 * x > width && 2 * y > height {
            blocks[3] += 1;
        }
    }
    println!("{:?}", blocks);
    println!("{:?}", blocks[0] * blocks[1] * blocks[2] * blocks[3]);
    for i in 0..height {
        for j in 0..width {
            print!("{}", grid[i as usize][j as usize]);
        }
        println!();
    }
}
#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut robots: Vec<Robot> = Vec::new();
    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let (x, y) = p.split_once(",").unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            let (_, v) = v.split_once("=").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();
            let vx = vx.parse::<i32>().unwrap();
            let vy = vy.parse::<i32>().unwrap();
            robots.push(Robot { x, y, vx, vy });
        });

    let width = 101;
    let height = 103;

    let mut i = 0;
    let mut found = false;
    while !found {
        let mut grid = vec![vec![0; 101]; 103];
        found = true;
        for robot in robots.iter() {
            let mut x = (robot.x + robot.vx * i) % width;
            let mut y = (robot.y + robot.vy * i) % height;
            if x < 0 {
                x += width;
            }
            if y < 0 {
                y += height;
            }
            grid[y as usize][x as usize] += 1;
            if grid[y as usize][x as usize] > 1 {
                found = false;
                break;
            }
        }
        i += 1;
        if found {
            for i in 0..height {
                for j in 0..width {
                    print!("{}", grid[i as usize][j as usize]);
                }
                println!();
            }
        }
    }

    println!("{:?}", i - 1);
}
