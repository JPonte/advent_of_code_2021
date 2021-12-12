use std::collections::HashMap;

use utils::*;

fn find_all_paths(
    paths_map: &HashMap<String, Vec<String>>,
    path: Vec<String>,
    max_visits: usize,
) -> Vec<Vec<String>> {
    paths_map
        .get(path.last().unwrap())
        .unwrap()
        .iter()
        .filter(|&next| {
            let counts = path
                .iter()
                .filter(|p| p.as_str() != "start" && p.chars().next().unwrap().is_lowercase())
                .fold(HashMap::from([(next.to_string(), 1)]), |mut acc, p| {
                    let entry = acc.entry(p.to_string()).or_insert(0);
                    *entry += 1;
                    acc
                });
            let repetitions = (max_visits == 1
                || counts.iter().filter(|(_, &v)| v == max_visits).count() <= 1)
                && counts.iter().filter(|(_, &v)| v > max_visits).count() == 0;

            (next.as_str() != "start")
                && (repetitions || next.chars().next().unwrap().is_uppercase())
        })
        .flat_map(|next| {
            let mut new_path = path.clone();
            new_path.push(next.to_string());
            if next == "end" {
                vec![new_path]
            } else {
                find_all_paths(paths_map, new_path, max_visits)
            }
        })
        .collect()
}

fn main() {
    let file = read_file("inputs/day_12/input.txt");

    let paths: Vec<(&str, &str)> = file
        .iter()
        .map(|line| {
            let split_line = line.split("-").collect::<Vec<&str>>();
            (*split_line.get(0).unwrap(), *split_line.get(1).unwrap())
        })
        .collect();

    let paths_map = paths.iter().fold(HashMap::new(), |mut acc, &(from, to)| {
        let tos = acc.entry(from.to_string()).or_insert(Vec::new());
        tos.push(to.to_string());

        let froms = acc.entry(to.to_string()).or_insert(Vec::new());
        froms.push(from.to_string());

        acc
    });

    let result = find_all_paths(&paths_map, vec!["start".to_string()], 1);
    println!("Result 1: {}", result.len());
    
    let result2 = find_all_paths(&paths_map, vec!["start".to_string()], 2);
    println!("Result 2: {}", result2.len());
}
