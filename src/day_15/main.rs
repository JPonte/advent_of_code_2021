use std::collections::HashMap;

use utils::*;

fn solve(map: &HashMap<(i32, i32), u32>) -> u32 {
    let (max_x, max_y) = *map.iter().max_by_key(|((x, y), _)| *x + *y).unwrap().0;

    let mut to_process = Vec::new();
    to_process.push((0, 0));

    let mut weights: HashMap<(i32, i32), u32> = HashMap::new();
    weights.insert((0, 0), 0);

    while !to_process.is_empty() {
        to_process.sort_by_key(|coords| -(*weights.get(coords).unwrap() as i32));
        let p = to_process.pop().unwrap();

        if weights
            .get(&(max_x, max_y))
            .filter(|&&w| w <= *weights.get(&p).unwrap())
            .is_some()
        {
            break;
        }

        let weight = weights.get(&p).unwrap();

        let neighboors: Vec<((i32, i32), u32)> = vec![
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0, p.1 - 1),
        ]
        .iter()
        .flat_map(|coords| {
            map.get(coords)
                .map(|n_weight| (*coords, *n_weight + *weight))
        })
        .filter(|(coords, w)| *w < *weights.get(coords).unwrap_or(&u32::MAX))
        .collect();

        neighboors.iter().for_each(|(coords, w)| {
            to_process.push(*coords);
            weights.insert(*coords, *w);
        });
    }
    *weights.get(&(max_x, max_y)).unwrap()
}

fn main() {
    let file = read_file("inputs/day_15/input.txt");

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

    let result1 = solve(&map);
    println!("Result 1: {}", result1);
    
    let (max_x, max_y) = *map.iter().max_by_key(|((x, y), _)| *x + *y).unwrap().0;

    let map2: HashMap<(i32, i32), u32> = map
        .iter()
        .flat_map(|(&(x, y), &weight)| {
            (0..5)
                .flat_map(|i| {
                    (0..5).map(move |j| ((x + ((max_x + 1) * i), y + ((max_y + 1) * j)), ((weight + (i + j) as u32) - 1) % 9 + 1))
                })
                .collect::<Vec<((i32, i32), u32)>>()
        })
        .collect();

    let result2 = solve(&map2);
    println!("Result 2: {}", result2);
}
