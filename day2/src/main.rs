use std::env;
use std::fs;

#[cfg(feature = "first")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let answer: i64 = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let report = line
                .split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if report.len() == 1 {
                return 1;
            }
            let inc = (report.first().unwrap() - report.last().unwrap()) > 0;
            for i in 0..report.len() - 1 {
                if inc && !(report[i] - report[i + 1] > 0 && report[i] - report[i + 1] < 4) {
                    return 0;
                }
                if !(inc || report[i + 1] - report[i] > 0 && report[i + 1] - report[i] < 4) {
                    return 0;
                }
            }
            1
        })
        .sum();
    println!("{}", answer);
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let answer: i64 = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let report = line
                .split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if report.len() <= 3 {
                return 1;
            }

            let dec = (report.first().unwrap() - report.last().unwrap()) > 0;
            let mut safe = true;
            for i in 0..report.len() - 1 {
                if dec && !(report[i] - report[i + 1] > 0 && report[i] - report[i + 1] < 4) {
                    safe = false;
                    break;
                }
                if !(dec || report[i + 1] - report[i] > 0 && report[i + 1] - report[i] < 4) {
                    safe = false;
                    break;
                }
            }
            if safe {
                return 1;
            }
            for i in 0..report.len() {
                let mut treport = report.clone();
                treport.remove(i);

                let dec = (treport.first().unwrap() - treport.last().unwrap()) > 0;
                let mut safe = true;
                for i in 0..treport.len() - 1 {
                    if dec && !(treport[i] - treport[i + 1] > 0 && treport[i] - treport[i + 1] < 4)
                    {
                        safe = false;
                        break;
                    }
                    if !(dec || treport[i + 1] - treport[i] > 0 && treport[i + 1] - treport[i] < 4)
                    {
                        safe = false;
                        break;
                    }
                }
                if safe {
                    return 1;
                }
            }
            0
        })
        .sum();
    println!("{}", answer);
}
