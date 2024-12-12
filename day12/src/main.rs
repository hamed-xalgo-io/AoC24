use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Pos(usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Edge(Pos, Pos);

#[cfg(feature = "second")]
fn calculate_price(plant: &[Pos], width: usize, height: usize) -> usize {
    use std::collections::HashSet;

    let area = plant.len();
    let mut perimeter = 0;
    let mut edges: HashSet<Edge> = HashSet::new();
    for Pos(x, y) in plant.iter() {
        if *x == 0 {
            perimeter += 1;
            edges.insert(Edge(Pos(*x, *y), Pos(*x, *y + 1)));
        }
        if *x == height - 1 {
            perimeter += 1;
            edges.insert(Edge(Pos(*x + 1, *y), Pos(*x + 1, *y + 1)));
        }
        if *y == 0 {
            perimeter += 1;
            edges.insert(Edge(Pos(*x, *y), Pos(*x + 1, *y)));
        }
        if *y == width - 1 {
            perimeter += 1;
            edges.insert(Edge(Pos(*x, *y + 1), Pos(*x + 1, *y + 1)));
        }
        if *x > 0 && !plant.contains(&Pos(x - 1, *y)) {
            perimeter += 1;
            edges.insert(Edge(Pos(*x, *y), Pos(*x, *y + 1)));
        }
        if *x < height - 1 && !plant.contains(&Pos(x + 1, *y)) {
            perimeter += 1;
            edges.insert(Edge(Pos(*x + 1, *y), Pos(*x + 1, *y + 1)));
        }
        if *y > 0 && !plant.contains(&Pos(*x, y - 1)) {
            perimeter += 1;
            edges.insert(Edge(Pos(*x, *y), Pos(*x + 1, *y)));
        }
        if *y < width - 1 && !plant.contains(&Pos(*x, y + 1)) {
            perimeter += 1;
            edges.insert(Edge(Pos(*x, *y + 1), Pos(*x + 1, *y + 1)));
        }
    }
    for edge in edges.iter() {
        if edge.0 .0 == edge.1 .0 {
            // equal x - horizontal
            let right_pos = &edge.1;
            let right_edge = Edge(right_pos.clone(), Pos(right_pos.0, right_pos.1 + 1));
            if edges.contains(&right_edge) {
                if right_pos.0 > 0 {
                    let up_edge = Edge(Pos(right_pos.0 - 1, right_pos.1), right_pos.clone());
                    let down_edge = Edge(right_pos.clone(), Pos(right_pos.0 + 1, right_pos.1));
                    if !(edges.contains(&up_edge) && edges.contains(&down_edge)) {
                        perimeter -= 1;
                    }
                } else {
                    perimeter -= 1;
                }
            }
        }
        if edge.0 .1 == edge.1 .1 {
            // equal y - vertical
            let bottom_pos = &edge.1;
            let bottom_edge = Edge(bottom_pos.clone(), Pos(bottom_pos.0 + 1, bottom_pos.1));
            if edges.contains(&bottom_edge) {
                if bottom_pos.1 > 0 {
                    let left_edge = Edge(Pos(bottom_pos.0, bottom_pos.1 - 1), bottom_pos.clone());
                    let right_edge = Edge(bottom_pos.clone(), Pos(bottom_pos.0, bottom_pos.1 + 1));
                    if !(edges.contains(&left_edge) && edges.contains(&right_edge)) {
                        perimeter -= 1;
                    }
                } else {
                    perimeter -= 1;
                }
            }
        }
    }
    perimeter * area
}

#[cfg(feature = "first")]
fn calculate_price(plant: &[Pos], width: usize, height: usize) -> usize {
    let area = plant.len();
    let mut perimeter = 0;

    for Pos(x, y) in plant.iter() {
        if *x == 0 || *x == height - 1 {
            perimeter += 1;
        }
        if *y == 0 || *y == width - 1 {
            perimeter += 1;
        }
        if *x > 0 && !plant.contains(&Pos(x - 1, *y)) {
            perimeter += 1;
        }
        if *x < height - 1 && !plant.contains(&Pos(x + 1, *y)) {
            perimeter += 1;
        }
        if *y > 0 && !plant.contains(&Pos(*x, y - 1)) {
            perimeter += 1;
        }
        if *y < width - 1 && !plant.contains(&Pos(*x, y + 1)) {
            perimeter += 1;
        }
    }

    perimeter * area
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut text: Vec<Vec<(char, bool)>> = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().map(|c| (c, false)).collect())
        .collect();
    let height = text.len();
    let width = text[0].len();
    let mut plants: Vec<Vec<Pos>> = vec![];
    for i in 0..text.len() {
        for j in 0..text[i].len() {
            if text[i][j].1 {
                continue;
            }
            let plant = text[i][j].0;
            let mut stack: Vec<Pos> = vec![];
            let mut connected: Vec<Pos> = vec![];
            stack.push(Pos(i, j));
            text[i][j].1 = true;
            while let Some(Pos(x, y)) = stack.pop() {
                connected.push(Pos(x, y));
                // text[x][y].1 = true;
                if x > 0 && text[x - 1][y].0 == plant && !text[x - 1][y].1 {
                    stack.push(Pos(x - 1, y));
                    text[x - 1][y].1 = true;
                }
                if x < text.len() - 1 && text[x + 1][y].0 == plant && !text[x + 1][y].1 {
                    stack.push(Pos(x + 1, y));
                    text[x + 1][y].1 = true;
                }
                if y > 0 && text[x][y - 1].0 == plant && !text[x][y - 1].1 {
                    stack.push(Pos(x, y - 1));
                    text[x][y - 1].1 = true;
                }
                if y < text[x].len() - 1 && text[x][y + 1].0 == plant && !text[x][y + 1].1 {
                    stack.push(Pos(x, y + 1));
                    text[x][y + 1].1 = true;
                }
            }
            plants.push(connected);
        }
    }
    let mut answer = 0;
    for plant in plants.iter() {
        answer += calculate_price(plant, width, height);
    }
    println!("{:?}", answer);
}
