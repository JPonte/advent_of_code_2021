use core::panic;
use std::collections::HashMap;

use utils::*;

fn parse_line(chars: &Vec<char>) -> (Vec<char>, Option<char>) {
    let corresponding_bracket = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut wrong_char = None;
    let mut stack = Vec::new();
    for &c in chars {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            ']' | ')' | '}' | '>' => {
                let pop_c = stack.pop();
                if pop_c
                    .filter(|x| c == *corresponding_bracket.get(x).unwrap())
                    .is_none()
                {
                    wrong_char = Some(c);
                    break;
                }
            }
            _ => panic!("oops"),
        }
    }
    (
        stack
            .iter()
            .rev()
            .map(|c| *corresponding_bracket.get(c).unwrap())
            .collect(),
        wrong_char,
    )
}

fn main() {
    let file = read_file("inputs/day_10/input.txt");

    let scoring = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let result: u64 = file
        .iter()
        .flat_map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let (_, wrong_char) = parse_line(&chars);
            wrong_char
        })
        .map(|c| scoring.get(&c).unwrap())
        .sum();

    println!("Result 1: {:?}", result);

    let scoring2 = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut scores: Vec<u64> = file
        .iter()
        .flat_map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let (reminding, wrong_char) = parse_line(&chars);
            if wrong_char.is_none() {
                Some(
                    reminding
                        .iter()
                        .map(|c| *scoring2.get(&c).unwrap())
                        .fold(0, |acc, c| acc * 5 + c),
                )
            } else {
                None
            }
        })
        .collect();

    scores.sort();

    let result2 = scores.get(scores.len() / 2).unwrap();

    println!("Result 2: {:?}", result2);
}
