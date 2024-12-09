use std::env;
use std::fs;

#[cfg(feature = "first")]
fn main() {
    use std::collections::VecDeque;

    let args: Vec<String> = env::args().collect();
    let line = fs::read_to_string(args[1].clone()).unwrap();
    let line = line.trim().chars().collect::<Vec<char>>();
    let mut answer = 0;
    let mut blocks: VecDeque<usize> = VecDeque::new();
    let mut spaces: VecDeque<usize> = VecDeque::new();
    for (i, num) in line.iter().enumerate() {
        let inum = num.to_string().parse::<usize>().unwrap();
        if i % 2 == 0 {
            blocks.push_back(inum);
        } else {
            spaces.push_back(inum);
        }
    }
    println!("{:?}", blocks.len());
    println!("{:?}", spaces.len());
    let mut left_id = 0usize;
    let mut pos = 0usize;
    let mut right_id = blocks.len() - 1;

    let mut flag = true;
    while !blocks.is_empty() && !spaces.is_empty() {
        if flag {
            flag = false;
            let block = blocks.pop_front().unwrap();
            for _ in 0..block {
                println!("{}*{}", left_id, pos);
                answer += left_id * pos;
                pos += 1;
            }
            left_id += 1;
        } else {
            let space = spaces.pop_front().unwrap();
            if space == 0 {
                flag = true;
                continue;
            }
            let block = blocks.pop_back().unwrap();
            if space >= block {
                spaces.push_front(space - block);

                for _ in 0..block {
                    println!("{}*{}", right_id, pos);
                    answer += right_id * pos;
                    pos += 1;
                }
                right_id -= 1;
            } else {
                blocks.push_back(block - space);
                for _ in 0..space {
                    println!("{}*{}", right_id, pos);
                    answer += right_id * pos;
                    pos += 1;
                }
                flag = true;
            }
        }
    }
    while !blocks.is_empty() {
        let block = blocks.pop_front().unwrap();
        for _ in 0..block {
            println!("{}*{}", left_id, pos);
            answer += left_id * pos;
            pos += 1;
        }
        left_id += 1;
    }
    println!("{}", answer);
}

#[cfg(feature = "second")]
#[derive(Debug, Clone)]
struct Block {
    id: usize,
    size: usize,
    file: bool,
    moved: bool,
}

#[cfg(feature = "second")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let line = fs::read_to_string(args[1].clone()).unwrap();
    let line = line.trim().chars().collect::<Vec<char>>();
    let mut blocks: Vec<Block> = Vec::new();
    for (i, num) in line.iter().enumerate() {
        let inum = num.to_string().parse::<usize>().unwrap();
        if i % 2 == 0 {
            blocks.push(Block {
                id: i / 2,
                size: inum,
                file: true,
                moved: false,
            });
        } else if inum != 0 {
            blocks.push(Block {
                id: 0,
                size: inum,
                file: false,
                moved: false,
            });
        }
    }
    let mut ready_to_move = blocks.len() - 1;
    while ready_to_move > 0 {
        let mut block = blocks[ready_to_move].clone();
        if !block.file || block.moved {
            ready_to_move -= 1;
            continue;
        }
        let mut moved = false;
        for i in 0..blocks.len() {
            if blocks[i].file {
                continue;
            }
            if i >= ready_to_move {
                break;
            }
            match blocks[i].size.cmp(&block.size) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => {
                    blocks[i].id = block.id;
                    blocks[i].file = true;
                    blocks[i].moved = true;
                    blocks[ready_to_move].file = false;
                    blocks[ready_to_move].id = 0;
                    ready_to_move -= 1;
                    moved = true;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    block.moved = true;
                    blocks[i].size -= block.size;
                    blocks[ready_to_move].file = false;
                    blocks[ready_to_move].id = 0;
                    blocks.insert(i, block);
                    moved = true;
                    break;
                }
            }
        }
        if moved {
            let mut i = 0;
            while i < blocks.len() - 1 {
                if !blocks[i].file && !blocks[i + 1].file {
                    blocks[i].size += blocks[i + 1].size;
                    blocks.remove(i + 1);
                } else {
                    i += 1;
                }
            }
        } else {
            ready_to_move -= 1;
        }
    }

    let mut answer: usize = 0;
    let mut pos = 0;
    for block in blocks.iter() {
        if block.file {
            for _ in 0..block.size {
                answer += block.id * pos;
                pos += 1;
            }
        } else {
            pos += block.size;
        }
    }

    println!("{:?}", answer);
}
