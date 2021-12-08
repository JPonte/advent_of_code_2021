use std::collections::{HashMap, HashSet};

use utils::*;

fn main() {
    let file = read_file("inputs/day_08/input.txt");

    let lines: Vec<Vec<Vec<&str>>> = file
        .iter()
        .map(|l| {
            l.split(" | ")
                .map(|part| part.split(" ").map(|x| x.clone()).collect())
                .collect()
        })
        .collect();

    let uniques = HashSet::from([2, 4, 3, 7]);
    let result: usize = lines
        .iter()
        .map(|line| {
            let output = line.get(1).unwrap();

            output
                .iter()
                .filter(|x| uniques.contains(&(x.len() as i32)))
                .count()
        })
        .sum();

    println!("Result 1: {}", result);

    let segments = vec![
        (0, HashSet::from(['a', 'b', 'c', 'e', 'f', 'g'])),
        (1, HashSet::from(['c', 'f'])),
        (2, HashSet::from(['a', 'c', 'd', 'e', 'g'])),
        (3, HashSet::from(['a', 'c', 'd', 'f', 'g'])),
        (4, HashSet::from(['b', 'c', 'd', 'f'])),
        (5, HashSet::from(['a', 'b', 'd', 'f', 'g'])),
        (6, HashSet::from(['a', 'b', 'd', 'e', 'f', 'g'])),
        (7, HashSet::from(['a', 'c', 'f'])),
        (8, HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
        (9, HashSet::from(['a', 'b', 'c', 'd', 'f', 'g'])),
    ];

    let letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    fn combine(s: Vec<char>, letters: &Vec<char>) -> Vec<Vec<char>> {
        if s.len() == 7 {
            vec![s]
        } else {
            let available: Vec<char> = letters
                .iter()
                .filter(|x| !s.contains(x))
                .map(|x| *x)
                .collect();
            available
                .iter()
                .flat_map(|x| {
                    let mut new_vec = s.clone();
                    new_vec.push(*x);
                    combine(new_vec, &available.clone())
                })
                .collect()
        }
    }

    let all_combs = combine(vec![], &letters);
    let all_maps: Vec<HashMap<&char, &char>> = all_combs
        .iter()
        .map(|comb| letters.iter().zip(comb).collect())
        .collect();

    let result2: i32 = lines
        .iter()
        .map(|line| {
            let input = line.get(0).unwrap();
            let output = line.get(1).unwrap();

            let correct_map: Vec<&HashMap<&char, &char>> = all_maps
                .iter()
                .filter(|map| {
                    let chars: Vec<HashSet<char>> = input
                        .iter()
                        .map(|s| s.chars().map(|x| **map.get(&x).unwrap()).collect())
                        .collect();

                    chars
                        .iter()
                        .find(|&x| segments.iter().find(|&y| *x == y.1).is_none())
                        .is_none()
                })
                .collect();

            output
                .iter()
                .rev()
                .enumerate()
                .map(|(i, s)| {
                    let set: HashSet<char> = s
                        .chars()
                        .map(|x| **correct_map.get(0).unwrap().get(&x).unwrap())
                        .collect();
                    segments.iter().find(|&x| x.1 == set).unwrap().0 * 10_i32.pow(i as u32)
                })
                .sum::<i32>()
        })
        .sum();

    println!("Result 2: {}", result2);
}
