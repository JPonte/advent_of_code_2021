use std::collections::HashSet;

use utils::*;

fn fold_horizontal(y: u32, dots: &HashSet<(u32, u32)>) -> HashSet<(u32, u32)> {
    let mut new_dots: HashSet<(u32, u32)> =
        dots.iter().filter(|&(_, dy)| *dy <= y).cloned().collect();

    dots.iter()
        .filter(|&(_, dy)| *dy > y)
        .for_each(|&(dx, dy)| {
            new_dots.insert((dx, y - (dy - y)));
        });
    new_dots
}

fn fold_vertical(x: u32, dots: &HashSet<(u32, u32)>) -> HashSet<(u32, u32)> {
    let mut new_dots: HashSet<(u32, u32)> =
        dots.iter().filter(|&(dx, _)| *dx <= x).cloned().collect();

    dots.iter()
        .filter(|&(dx, _)| *dx > x)
        .for_each(|&(dx, dy)| {
            new_dots.insert((x - (dx - x), dy));
        });
    new_dots
}

fn main() {
    let file = read_file("inputs/day_13/input.txt");

    let mut dots: HashSet<(u32, u32)> = HashSet::new();
    let mut folds: Vec<(String, u32)> = Vec::new();
    let mut part = 0;
    for line in file {
        if line.as_str() == "" {
            part += 1;
            continue;
        }
        if part == 0 {
            let split_line = line.split(",").collect::<Vec<&str>>();
            dots.insert((
                split_line.get(0).unwrap().parse().unwrap(),
                split_line.get(1).unwrap().parse().unwrap(),
            ));
        } else if part == 1 {
            let split_line = line[11..].split("=").collect::<Vec<&str>>();
            folds.push((
                split_line.get(0).unwrap().to_string(),
                split_line.get(1).unwrap().parse().unwrap(),
            ));
        }
    }

    if let Some((dir, n)) = folds.get(0) {
        if dir.as_str() == "x" {
            dots = fold_vertical(*n, &dots);
        } else {
            dots = fold_horizontal(*n, &dots);
        }
    }

    println!("Result 1: {}", dots.len());

    folds.iter().skip(1).for_each(|(dir, n)| {
        if dir.as_str() == "x" {
            dots = fold_vertical(*n, &dots);
        } else {
            dots = fold_horizontal(*n, &dots);
        }
    });

    println!("Result 2:");
    let height = dots.iter().max_by_key(|&(_, dy)| *dy).unwrap().1 + 1;
    let width = dots.iter().max_by_key(|&(dx, _)| *dx).unwrap().0 + 1;
    for y in 0..height {
        for x in 0..width {
            if dots.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("")
    }
}
