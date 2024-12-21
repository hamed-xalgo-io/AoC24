use std::env;
use std::{collections::HashMap, fs};

use itertools::Itertools;
use pathfinding::prelude::astar_bag_collect;

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/

fn numpad_build_path(seq: &[char]) -> Vec<char> {
    let move_map: HashMap<(char, char), char> = vec![
        (('A', '0'), '<'),
        (('A', '3'), '^'),
        (('0', '2'), '^'),
        (('0', 'A'), '>'),
        (('1', '2'), '>'),
        (('1', '4'), '^'),
        (('2', '1'), '<'),
        (('2', '3'), '>'),
        (('2', '5'), '^'),
        (('2', '0'), 'v'),
        (('3', '2'), '<'),
        (('3', '6'), '^'),
        (('3', 'A'), 'v'),
        (('4', '1'), 'v'),
        (('4', '5'), '>'),
        (('4', '7'), '^'),
        (('5', '2'), 'v'),
        (('5', '4'), '<'),
        (('5', '6'), '>'),
        (('5', '8'), '^'),
        (('6', '3'), 'v'),
        (('6', '5'), '<'),
        (('6', '9'), '^'),
        (('7', '4'), 'v'),
        (('7', '8'), '>'),
        (('8', '5'), 'v'),
        (('8', '7'), '<'),
        (('8', '9'), '>'),
        (('9', '6'), 'v'),
        (('9', '8'), '<'),
    ]
    .into_iter()
    .collect();
    let mut path = vec![];
    for i in 0..seq.len() - 1 {
        path.push(move_map[&(seq[i], seq[i + 1])]);
    }
    path
}

fn numpad_successors(p: &char) -> Vec<(char, i32)> {
    match p {
        'A' => vec![('0', 1), ('3', 1)],
        '0' => vec![('2', 1), ('A', 1)],
        '1' => vec![('2', 1), ('4', 1)],
        '2' => vec![('1', 1), ('3', 1), ('5', 1), ('0', 1)],
        '3' => vec![('2', 1), ('6', 1), ('A', 1)],
        '4' => vec![('1', 1), ('5', 1), ('7', 1)],
        '5' => vec![('2', 1), ('4', 1), ('6', 1), ('8', 1)],
        '6' => vec![('3', 1), ('5', 1), ('9', 1)],
        '7' => vec![('4', 1), ('8', 1)],
        '8' => vec![('5', 1), ('7', 1), ('9', 1)],
        '9' => vec![('6', 1), ('8', 1)],
        _ => unreachable!("Invalid numpad"),
    }
}
fn numpad_all_possible_moves() -> HashMap<(char, char), Vec<String>> {
    let mut possible_moves: HashMap<(char, char), Vec<String>> = HashMap::new();
    let numpad = "A0123456789";
    numpad.chars().tuple_combinations().for_each(|(s, d)| {
        let s_to_d = astar_bag_collect(&s, numpad_successors, |_| 0, |p| *p == d)
            .unwrap()
            .0;
        possible_moves.insert(
            (s, d),
            s_to_d
                .iter()
                .map(|seq| numpad_build_path(seq).iter().collect::<String>())
                .collect::<Vec<_>>(),
        );
        let d_to_s = astar_bag_collect(&d, numpad_successors, |_| 0, |p| *p == s)
            .unwrap()
            .0;
        possible_moves.insert(
            (d, s),
            d_to_s
                .iter()
                .map(|seq| numpad_build_path(seq).iter().collect::<String>())
                .collect::<Vec<_>>(),
        );
    });
    possible_moves
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
 */
fn dirpad_build_path(seq: &[char]) -> Vec<char> {
    let move_map: HashMap<(char, char), char> = vec![
        (('A', '>'), 'v'),
        (('A', '^'), '<'),
        (('^', 'A'), '>'),
        (('^', 'v'), 'v'),
        (('v', '>'), '>'),
        (('v', '^'), '^'),
        (('v', '<'), '<'),
        (('>', 'v'), '<'),
        (('>', 'A'), '^'),
        (('<', 'v'), '>'),
    ]
    .into_iter()
    .collect();
    let mut path = vec![];
    for i in 0..seq.len() - 1 {
        path.push(move_map[&(seq[i], seq[i + 1])]);
    }
    path
}

fn dirpad_successors(p: &char) -> Vec<(char, i32)> {
    match p {
        'A' => vec![('^', 1), ('>', 1)],
        '^' => vec![('A', 1), ('v', 1)],
        '>' => vec![('A', 1), ('v', 1)],
        'v' => vec![('^', 1), ('>', 1), ('<', 1)],
        '<' => vec![('v', 1)],
        _ => unreachable!("Invalid numpad"),
    }
}

fn dirpad_all_possible_moves() -> HashMap<(char, char), Vec<String>> {
    let mut possible_moves: HashMap<(char, char), Vec<String>> = HashMap::new();
    let dirpad = "A^>v<";
    dirpad.chars().tuple_combinations().for_each(|(s, d)| {
        let s_to_d = astar_bag_collect(&s, dirpad_successors, |_| 0, |p| *p == d)
            .unwrap()
            .0;
        possible_moves.insert(
            (s, d),
            s_to_d
                .iter()
                .map(|seq| dirpad_build_path(seq).iter().collect::<String>())
                .collect::<Vec<_>>(),
        );
        let d_to_s = astar_bag_collect(&d, dirpad_successors, |_| 0, |p| *p == s)
            .unwrap()
            .0;
        possible_moves.insert(
            (d, s),
            d_to_s
                .iter()
                .map(|seq| dirpad_build_path(seq).iter().collect::<String>())
                .collect::<Vec<_>>(),
        );
    });
    possible_moves
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let passcodes = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();
    let numpad_possible_moves = numpad_all_possible_moves();
    let dirpad_possible_moves = dirpad_all_possible_moves();
    let mut answer = 0;
    for passcode in passcodes {
        let passcode = "A".to_string() + &passcode;
        let mut paths = passcode
            .chars()
            .tuple_windows()
            .map(|(s, d)| {
                numpad_possible_moves[&(s, d)]
                    .iter()
                    .map(|moves| format!("{moves}A"))
            })
            .multi_cartesian_product()
            .map(|path| "A".to_string() + &path.join(""))
            .collect::<Vec<_>>();
        let mut optimal_len = paths.iter().map(|path| path.len()).min().unwrap();
        paths.retain(|path| path.len() == optimal_len);
        for _ in 0..2 {
            paths = paths
                .iter()
                .flat_map(|path| {
                    path.chars()
                        .tuple_windows()
                        .map(|(s, d)| {
                            if s == d {
                                vec!["A".to_string()]
                            } else {
                                dirpad_possible_moves[&(s, d)]
                                    .iter()
                                    .map(|moves| format!("{moves}A"))
                                    .collect::<Vec<_>>()
                            }
                        })
                        .multi_cartesian_product()
                        .map(|path| "A".to_string() + &path.join(""))
                })
                .collect();

            optimal_len = paths.iter().map(|path| path.len()).min().unwrap();
            paths.retain(|path| path.len() == optimal_len);
        }

        answer += (optimal_len - 1)
            * passcode
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
    }
    println!("Answer: {}", answer);
}
