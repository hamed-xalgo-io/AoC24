use std::env;
use std::fs;

#[cfg(feature = "first")]
fn check_xmas(text: &[Vec<char>], i: usize, j: usize) -> i32 {
    let mut count = 0;
    if j >= 3 && text[i][j - 1] == 'M' && text[i][j - 2] == 'A' && text[i][j - 3] == 'S' {
        count += 1;
    }
    if j + 3 < text[i].len()
        && text[i][j + 1] == 'M'
        && text[i][j + 2] == 'A'
        && text[i][j + 3] == 'S'
    {
        count += 1;
    }
    if i >= 3 && text[i - 1][j] == 'M' && text[i - 2][j] == 'A' && text[i - 3][j] == 'S' {
        count += 1;
    }
    if i + 3 < text.len() && text[i + 1][j] == 'M' && text[i + 2][j] == 'A' && text[i + 3][j] == 'S'
    {
        count += 1;
    }
    if i >= 3
        && j >= 3
        && text[i - 1][j - 1] == 'M'
        && text[i - 2][j - 2] == 'A'
        && text[i - 3][j - 3] == 'S'
    {
        count += 1;
    }
    if i + 3 < text.len()
        && j + 3 < text[i].len()
        && text[i + 1][j + 1] == 'M'
        && text[i + 2][j + 2] == 'A'
        && text[i + 3][j + 3] == 'S'
    {
        count += 1;
    }
    if i >= 3
        && j + 3 < text[i].len()
        && text[i - 1][j + 1] == 'M'
        && text[i - 2][j + 2] == 'A'
        && text[i - 3][j + 3] == 'S'
    {
        count += 1;
    }
    if i + 3 < text.len()
        && j >= 3
        && text[i + 1][j - 1] == 'M'
        && text[i + 2][j - 2] == 'A'
        && text[i + 3][j - 3] == 'S'
    {
        count += 1;
    }

    count
}

#[cfg(feature = "second")]
fn check_x_mas(text: &[Vec<char>], i: usize, j: usize) -> i32 {
    let mut count = 0;
    if i >= 1 && i + 1 < text.len() && j >= 1 && j + 1 < text[i].len() {
        if text[i - 1][j - 1] == 'M'
            && text[i - 1][j + 1] == 'M'
            && text[i + 1][j - 1] == 'S'
            && text[i + 1][j + 1] == 'S'
        {
            count += 1;
        }

        if text[i - 1][j - 1] == 'S'
            && text[i - 1][j + 1] == 'S'
            && text[i + 1][j - 1] == 'M'
            && text[i + 1][j + 1] == 'M'
        {
            count += 1;
        }

        if text[i - 1][j - 1] == 'S'
            && text[i - 1][j + 1] == 'M'
            && text[i + 1][j - 1] == 'S'
            && text[i + 1][j + 1] == 'M'
        {
            count += 1;
        }

        if text[i - 1][j - 1] == 'M'
            && text[i - 1][j + 1] == 'S'
            && text[i + 1][j - 1] == 'M'
            && text[i + 1][j + 1] == 'S'
        {
            count += 1;
        }
    }
    count
}

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let text: Vec<Vec<char>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut answer = 0;
    for i in 0..text.len() {
        for j in 0..text[i].len() {
            if text[i][j] == 'X' {
                answer += check_xmas(&text, i, j);
            }
        }
    }
    println!("{:?}", answer);
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let text: Vec<Vec<char>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut answer = 0;
    for i in 0..text.len() {
        for j in 0..text[i].len() {
            if text[i][j] == 'A' {
                answer += check_x_mas(&text, i, j);
            }
        }
    }
    println!("{:?}", answer);
}
