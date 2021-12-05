use std::collections::HashMap;

use utils::*;

fn count_intersections(lines: &Vec<Vec<(u32, u32)>>) -> u32 {
    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();
    lines.iter().flat_map(|x| x).for_each(|coord| {
        let new_count = match counts.get(coord) {
            Some(count) => count + 1,
            None => 1,
        };
        counts.insert(*coord, new_count);
    });
    counts.iter().filter(|(_, &v)| v > 1).count() as u32
}

fn main() {
    let file = read_file("inputs/day_05/input.txt");

    let coords: Vec<((u32, u32), (u32, u32))> = file
        .iter()
        .map(|l| {
            let coords: Vec<&str> = l.split(" -> ").collect();
            let v1: Vec<&str> = coords[0].split(",").collect();
            let v2: Vec<&str> = coords[1].split(",").collect();
            let (x1, y1): (u32, u32) = (v1[0].parse().unwrap(), v1[1].parse().unwrap());
            let (x2, y2): (u32, u32) = (v2[0].parse().unwrap(), v2[1].parse().unwrap());
            ((x1, y1), (x2, y2))
        })
        .collect();

    let lines: Vec<Vec<(u32, u32)>> = coords
        .iter()
        .map(|((x1, y1), (x2, y2))| {
            if *x1 == *x2 {
                if *y1 < *y2 { *y1..=*y2 } else { *y2..=*y1 }
                    .map(|y| (*x1, y))
                    .collect()
            } else if *y1 == *y2 {
                if *x1 < *x2 { *x1..=*x2 } else { *x2..=*x1 }
                    .map(|x| (x, *y1))
                    .collect()
            } else {
                Vec::new()
            }
        })
        .collect();

    let result = count_intersections(&lines);
    println!("Result 1: {}", result);

    let lines2: Vec<Vec<(u32, u32)>> = coords
        .iter()
        .map(|((x1, y1), (x2, y2))| {
            if *x1 == *x2 {
                if *y1 < *y2 { *y1..=*y2 } else { *y2..=*y1 }
                    .map(|y| (*x1, y))
                    .collect()
            } else if *y1 == *y2 {
                if *x1 < *x2 { *x1..=*x2 } else { *x2..=*x1 }
                    .map(|x| (x, *y1))
                    .collect()
            } else {
                let ys: Vec<u32> = if *y1 < *y2 {
                    (*y1..=*y2).collect()
                } else {
                    (*y2..=*y1).rev().collect()
                };
                let xs: Vec<u32> = if *x1 < *x2 {
                    (*x1..=*x2).collect()
                } else {
                    (*x2..=*x1).rev().collect()
                };
                let res = xs.iter().map(|x| x.clone()).zip(ys).collect();
                res
            }
        })
        .collect();

    let result2 = count_intersections(&lines2);
    println!("Result 2: {}", result2);
}
