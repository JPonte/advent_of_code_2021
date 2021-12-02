#![feature(mixed_integer_ops)]

use utils::*;

fn main() {
    let file = read_file("inputs/day_02/input.txt");

    let commands: Vec<(&str, u32)> = file
        .iter()
        .map(|l| {
            let split: Vec<&str> = l.split(" ").collect();
            (split[0], split[1].parse().unwrap())
        })
        .collect();

    let mut horizontal_pos: u32 = 0;
    let mut depth: u32 = 0;

    for (command, amount) in commands.iter() {
        match *command {
            "forward" => horizontal_pos += *amount,
            "up" => depth = depth.saturating_sub(*amount),
            "down" => depth += *amount,
            _ => {}
        }
    }
    println!("First: {}", horizontal_pos * depth);

    let mut aim: i32 = 0;
    horizontal_pos = 0;
    depth = 0;

    for (command, amount) in commands.iter() {
        match *command {
            "forward" => {
                horizontal_pos += *amount;
                depth = depth.saturating_add_signed(aim * (*amount as i32));
            }
            "up" => aim -= *amount as i32,
            "down" => aim += *amount as i32,
            _ => {}
        }
    }
    println!("Second: {}", horizontal_pos * depth);
}
