use std::env;
use std::fs;

#[cfg(feature = "first")]
fn main() {
    use std::collections::HashSet;

    let args: Vec<String> = env::args().collect();
    let text: Vec<Vec<char>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let width = text[0].len();
    let height = text.len();
    let depth = 10usize;
    let mut answer: Vec<Vec<Vec<HashSet<(usize, usize)>>>> =
        vec![vec![vec![HashSet::new(); width]; height]; depth];
    for r in 0..height {
        for c in 0..width {
            if text[r][c] == '0' {
                answer[0][r][c].insert((r, c));
            }
        }
    }

    let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for d in 1..depth {
        for r in 0..height {
            for c in 0..width {
                if text[r][c] == chars[d] {
                    answer[d][r][c] = HashSet::new();
                    if r > 0 {
                        let temp = answer[d - 1][r - 1][c].clone();
                        answer[d][r][c].extend(temp);
                    }
                    if r < height - 1 {
                        let temp = answer[d - 1][r + 1][c].clone();
                        answer[d][r][c].extend(temp);
                    }
                    if c > 0 {
                        let temp = answer[d - 1][r][c - 1].clone();
                        answer[d][r][c].extend(temp);
                    }
                    if c < width - 1 {
                        let temp = answer[d - 1][r][c + 1].clone();
                        answer[d][r][c].extend(temp);
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for r in 0..height {
        for c in 0..width {
            sum += answer[9][r][c].len();
        }
    }
    println!("{:?}", sum);
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let text: Vec<Vec<char>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let width = text[0].len();
    let height = text.len();
    let depth = 10usize;
    let mut answer: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; width]; height]; depth];
    for r in 0..height {
        for c in 0..width {
            if text[r][c] == '0' {
                answer[0][r][c] = 1;
            }
        }
    }

    let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for d in 1..depth {
        for r in 0..height {
            for c in 0..width {
                if text[r][c] == chars[d] {
                    answer[d][r][c] = 0;
                    if r > 0 {
                        answer[d][r][c] += answer[d - 1][r - 1][c];
                    }
                    if r < height - 1 {
                        answer[d][r][c] += answer[d - 1][r + 1][c];
                    }
                    if c > 0 {
                        answer[d][r][c] += answer[d - 1][r][c - 1];
                    }
                    if c < width - 1 {
                        answer[d][r][c] += answer[d - 1][r][c + 1];
                    }
                }
            }
        }
    }
    let answer = answer[9]
        .iter()
        .map(|row| row.iter().sum::<u32>())
        .sum::<u32>();
    println!("{:?}", answer);
}
