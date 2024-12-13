use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

#[cfg(feature = "first")]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

#[cfg(feature = "first")]
impl Pos {
    fn successors(&self, machine: &Machine) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        if x > machine.prize.0 || y > machine.prize.1 {
            return vec![];
        }
        vec![
            (Pos(x + machine.a.0, y + machine.a.1), 3),
            (Pos(x + machine.b.0, y + machine.b.1), 1),
        ]
    }
}

#[cfg(feature = "first")]
fn process_instruction(machine: &Machine) -> usize {
    use pathfinding::prelude::dijkstra;

    let goal = Pos(machine.prize.0, machine.prize.1);
    let start = Pos(0, 0);
    let result = dijkstra(&start, |p| p.successors(machine), |p| *p == goal);
    if let Some(result) = result {
        result.1
    } else {
        0
    }
}

#[cfg(feature = "second")]
fn process_instruction(machine: &Machine) -> usize {
    let a0 = machine.a.0 as i64;
    let a1 = machine.a.1 as i64;
    let b0 = machine.b.0 as i64;
    let b1 = machine.b.1 as i64;
    let p0 = machine.prize.0 as i64;
    let p1 = machine.prize.1 as i64;
    let b = (a0 * p1 - a1 * p0) / (a0 * b1 - a1 * b0);
    let br = (a0 * p1 - a1 * p0) % (a0 * b1 - a1 * b0);
    let a = (p0 - b * b0) / a0;
    let ar = (p0 - b * b0) % a0;
    if ar != 0 || br != 0 {
        return 0;
    }
    (a * 3 + b) as usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut machines: Vec<Machine> = Vec::new();
    let mut machine = Machine {
        a: (0, 0),
        b: (0, 0),
        prize: (0, 0),
    };

    fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            if idx % 4 < 2 {
                let a = line.split_once(":").unwrap().1;
                let (x, y) = a.split_once(",").unwrap();
                let x = x.split_once("+").unwrap().1.parse::<usize>().unwrap();
                let y = y.split_once("+").unwrap().1.parse::<usize>().unwrap();
                if idx % 4 == 0 {
                    machine.a = (x, y);
                } else {
                    machine.b = (x, y);
                }
            }
            if idx % 4 == 2 {
                let p = line.split_once(":").unwrap().1;
                let (x, y) = p.split_once(",").unwrap();
                let x = x.split_once("=").unwrap().1.parse::<usize>().unwrap();
                let y = y.split_once("=").unwrap().1.parse::<usize>().unwrap();
                machine.prize = (x, y);
                if cfg!(feature = "second") {
                    machine.prize.0 += 10000000000000;
                    machine.prize.1 += 10000000000000;
                }
                machines.push(machine.clone());
            }
        });
    let answer: usize = machines.iter().map(process_instruction).sum();
    println!("{:?}", answer);
}
