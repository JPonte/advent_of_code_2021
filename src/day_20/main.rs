use std::collections::HashSet;

use utils::*;

fn kernel(image: &HashSet<(i32, i32)>, x: i32, y: i32) -> usize {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .iter()
    .map(|k| image.contains(k))
    .rev()
    .enumerate()
    .map(|(i, light)| if light { 2_usize.pow(i as u32) } else { 0 })
    .sum()
}

fn enhance(
    image: &HashSet<(i32, i32)>,
    enhancer: &Vec<bool>,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
) -> HashSet<(i32, i32)> {
    let mut new_image = HashSet::new();

    for px in min_x..max_x {
        for py in min_y..max_y {
            let k_i = kernel(&image, px, py);
            if *enhancer.get(k_i).unwrap() {
                new_image.insert((px, py));
            }
        }
    }

    new_image
}

fn solve(image: &HashSet<(i32, i32)>, enhancer: &Vec<bool>, rounds: u32) -> usize {
    let max_x = image.iter().max_by_key(|(x, _)| *x).unwrap().0 + rounds as i32 * 2 + 1;
    let max_y = image.iter().max_by_key(|(_, y)| *y).unwrap().1 + rounds as i32 * 2 + 1;
    let min_x = image.iter().min_by_key(|(x, _)| *x).unwrap().0 - rounds as i32 * 2;
    let min_y = image.iter().min_by_key(|(_, y)| *y).unwrap().1 - rounds as i32 * 2;

    let result = (0..rounds).fold(image.clone(), |acc, turn| {
        enhance(
            &acc,
            &enhancer,
            max_x - turn as i32,
            max_y - turn as i32,
            min_x + turn as i32,
            min_y + turn as i32,
        )
    });
    result.len()
}

fn main() {
    let file = read_file("inputs/day_20/input.txt");
    let enhancer: Vec<bool> = file.get(0).unwrap().chars().map(|c| c == '#').collect();

    let image: HashSet<(i32, i32)> = file
        .iter()
        .skip(2)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| ((x as i32, y as i32)))
                .collect::<HashSet<(i32, i32)>>()
        })
        .collect();


    println!("Result 1: {}", solve(&image, &enhancer, 2));

    println!("Result 2: {}", solve(&image, &enhancer, 50));
}
