use core::panic;
use std::env;
use std::fs;

fn combo(arg: usize, ra: usize, rb: usize, rc: usize) -> usize {
    match arg {
        0..=3 => arg,
        4 => ra,
        5 => rb,
        6 => rc,
        _ => panic!("Invalid argument"),
    }
}

fn run(
    prog: &[usize],
    i: usize,
    ra: &mut usize,
    rb: &mut usize,
    rc: &mut usize,
) -> (usize, Option<usize>) {
    let op = prog[i];
    let arg = prog[i + 1];
    match op {
        0 => {
            *ra = *ra / 2_usize.pow(combo(arg, *ra, *rb, *rc) as u32);
            (i + 2, None)
        }
        1 => {
            *rb ^= arg;
            (i + 2, None)
        }
        2 => {
            *rb = combo(arg, *ra, *rb, *rc) % 8;
            (i + 2, None)
        }
        3 => {
            if *ra != 0 {
                (arg, None)
            } else {
                (i + 2, None)
            }
        }
        4 => {
            *rb ^= *rc;
            (i + 2, None)
        }
        5 => (i + 2, Some(combo(arg, *ra, *rb, *rc) % 8)),
        6 => {
            *rb = *ra / 2_usize.pow(combo(arg, *ra, *rb, *rc) as u32);
            (i + 2, None)
        }
        7 => {
            *rc = *ra / 2_usize.pow(combo(arg, *ra, *rb, *rc) as u32);
            (i + 2, None)
        }
        _ => {
            panic!("Invalid opcode");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let mut ra = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut rb = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut rc = lines[2]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let (_, prog) = lines[4].split_once(" ").unwrap();
    let prog = prog
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut i = 0;
    let mut output: Vec<usize> = Vec::new();
    while i < prog.len() {
        let (ni, out) = run(&prog, i, &mut ra, &mut rb, &mut rc);
        if let Some(out) = out {
            output.push(out);
        }
        i = ni;
    }
    println!(
        "{:?}",
        output
            .into_iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
