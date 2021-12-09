use std::collections::{HashMap, HashSet};

use utils::*;

fn find_basin(map: &HashMap<(i32, i32), u32>, acc: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_acc: HashSet<(i32, i32)> = acc
        .iter()
        .flat_map(|&(x, y)| {
            vec![(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .flat_map(|coords| map.get(coords).map(|&d| (*coords, d)))
                .filter(|&(_, d)| d != 9 && d >= *(map.get(&(x, y)).unwrap()))
                .map(|(c, _)| c)
                .collect::<Vec<(i32, i32)>>()
        })
        .collect();

    new_acc = new_acc.union(&acc).cloned().collect();
    if new_acc != acc {
        find_basin(map, new_acc)
    } else {
        acc
    }
}

fn main() {
    let file = read_file("inputs/day_09/input.txt");

    let map: HashMap<(i32, i32), u32> = file
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
                .collect::<HashMap<(i32, i32), u32>>()
        })
        .collect();

    let mins: Vec<(i32, i32)> = map
        .iter()
        .flat_map(|(&(x, y), depth)| {
            let neighbours: Vec<&u32> = vec![
                map.get(&(x - 1, y)),
                map.get(&(x + 1, y)),
                map.get(&(x, y + 1)),
                map.get(&(x, y - 1)),
            ]
            .iter()
            .flat_map(|x| *x)
            .collect();
            if neighbours.iter().find(|&&n| *depth >= *n).is_none() {
                Some((x, y))
            } else {
                None
            }
        })
        .collect();

    let result: u32 = mins.iter().map(|min| map.get(&min).unwrap() + 1).sum();

    println!("Result 1: {}", result);

    let mut basins: Vec<HashSet<(i32, i32)>> = mins
        .iter()
        .map(|&min| find_basin(&map, HashSet::from([min])))
        .collect();

    basins.sort_by_key(|basin| 0 - basin.len() as i32);

    let result2 = basins
        .iter()
        .take(3)
        .fold(1, |acc, basin| acc * basin.len());

    println!("Result 2: {}", result2);
}
