use std::env;
use std::fs;

fn valid_calibration(nums: &[u64], target: u64) -> bool {
    let mut queue = vec![(&nums[1..], nums[0])];
    while let Some((nums, acc)) = queue.pop() {
        if nums.is_empty() && acc == target {
            return true;
        }
        if nums.is_empty() {
            continue;
        }
        let head = nums[0];
        queue.push((&nums[1..], acc + head));
        queue.push((&nums[1..], acc * head));
        #[cfg(feature = "second")]
        queue.push((&nums[1..], format!("{}{}", acc, head).parse().unwrap()));
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let answer = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let (sum, nums) = line.split_once(":").unwrap();
            let sum = sum.trim().parse().unwrap();
            let nums: Vec<u64> = nums
                .split_ascii_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect();
            if valid_calibration(&nums, sum) {
                sum
            } else {
                0
            }
        })
        .sum::<u64>();

    println!("{:?}", answer);
}
