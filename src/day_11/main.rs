use std::collections::HashMap;

use utils::*;

fn main() {
    let file = read_file("inputs/day_11/input.txt");

    let mut map: HashMap<(i32, i32), u32> = file
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
                .collect::<HashMap<(i32, i32), u32>>()
        })
        .collect();

    let mut result1 = 0;
    let mut i = 0;
    let mut turn_flashes = 0;
    while turn_flashes != 100 {
        let mut flashes = HashMap::new();

        map.iter_mut().for_each(|(&coords, energy)| {
            *energy += 1;
            if *energy > 9 {
                flashes.insert(coords, 1);
            }
        });

        let mut flash_gen = 1;
        let mut flashes_added = true;
        while flashes_added {
            flashes_added = false;
            map.iter_mut().for_each(|(&(x, y), energy)| {
                if *energy <= 9 {
                    let neighbours = vec![
                        (x - 1, y),
                        (x - 1, y + 1),
                        (x - 1, y - 1),
                        (x + 1, y),
                        (x + 1, y + 1),
                        (x + 1, y - 1),
                        (x, y + 1),
                        (x, y - 1),
                    ];
                    let enery_add = neighbours
                        .iter()
                        .filter(|n| flashes.get(n).filter(|&f| *f == flash_gen).is_some())
                        .count();
                    *energy += enery_add as u32;
                    if *energy > 9 {
                        flashes.insert((x, y), flash_gen + 1);
                        flashes_added = true;
                    }
                }
            });
            flash_gen += 1;
        }

        map.iter_mut().for_each(|(_, energy)| {
            if *energy > 9 {
                *energy = 0;
            }
        });

        turn_flashes = flashes.len();
        if i < 100 {
            result1 += turn_flashes;
        }
        i += 1;
    }
    println!("Result 1: {}", result1);
    println!("Result 2: {}", i);
}
