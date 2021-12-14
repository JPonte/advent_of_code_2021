use std::collections::HashMap;
use utils::*;

fn iteration(
    sequence_map: &HashMap<String, u64>,
    pair_map: &HashMap<String, String>,
) -> HashMap<String, u64> {
    sequence_map
        .iter()
        .flat_map(|(pair, count)| {
            if let Some(m) = pair_map.get(pair) {
                let new_pair_1 = format!("{}{}", &pair[0..1], m);
                let new_pair_2 = format!("{}{}", m, &pair[1..2]);
                vec![(new_pair_1, count), (new_pair_2, count)]
            } else {
                vec![]
            }
        })
        .fold(HashMap::new(), |mut acc, (pair, count)| {
            let entry = acc.entry(pair).or_insert(0);
            *entry += count;
            acc
        })
}

fn count_result(sequence_map: &HashMap<String, u64>) -> u64 {
    let counts = sequence_map
        .iter()
        .fold(HashMap::new(), |mut acc, (pair, count)| {
            let entry1 = acc.entry(&pair[0..1]).or_insert(0);
            *entry1 += count;
            let entry2 = acc.entry(&pair[1..2]).or_insert(0);
            *entry2 += count;
            acc
        });
    let max = counts.iter().max_by_key(|x| x.1).unwrap().1;
    let min = counts.iter().min_by_key(|x| x.1).unwrap().1;
    (max / 2 + max % 2) - (min / 2 + min % 2)
}

fn main() {
    let file = read_file("inputs/day_14/input.txt");
    let sequence = file.get(0).unwrap().clone();

    let pair_map: HashMap<String, String> = file
        .iter()
        .skip(2)
        .map(|line| {
            let pair = line.split(" -> ").collect::<Vec<&str>>();
            (
                pair.get(0).unwrap().to_string(),
                pair.get(1).unwrap().to_string(),
            )
        })
        .collect();

    let mut sequence_map: HashMap<String, u64> = HashMap::new();

    (0..sequence.len() - 1).for_each(|i| {
        let pair = sequence[i..i + 2].to_string();
        let entry = sequence_map.entry(pair).or_insert(0);
        *entry += 1;
    });

    for _ in 0..10 {
        sequence_map = iteration(&sequence_map, &pair_map);
    }

    println!("Result 1: {}", count_result(&sequence_map));

    for _ in 10..40 {
        sequence_map = iteration(&sequence_map, &pair_map);
    }

    println!("Result 2: {}", count_result(&sequence_map));
}
