use std::collections::HashMap;
use utils::*;

fn solve_2(p1: u32, p2: u32, score1: u64, score2: u64, turn: bool) -> (u64, u64) {
    if score1 >= 21 {
        (1, 0)
    } else if score2 >= 21 {
        (0, 1)
    } else {
        vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
            .iter()
            .map(|&(d, c)| {
                if turn {
                    let new_p1 = (p1 - 1 + d) % 10 + 1;
                    let new_score_1 = score1 + new_p1 as u64;
                    (c, solve_2(new_p1, p2, new_score_1, score2, !turn))
                } else {
                    let new_p2 = (p2 - 1 + d) % 10 + 1;
                    let new_score_2 = score2 + new_p2 as u64;
                    (c, solve_2(p1, new_p2, score1, new_score_2, !turn))
                }
            })
            .fold((0, 0), |(acc1, acc2), (c, (s1, s2))| {
                (s1 * c + acc1, s2 * c + acc2)
            })
    }
}

fn main() {
    let file = read_file("inputs/day_21/input.txt");

    let mut positions: HashMap<u32, u32> = file
        .iter()
        .map(|line| {
            let splits = line.split(" ").collect::<Vec<&str>>();
            (
                splits.get(1).unwrap().parse().unwrap(),
                splits.get(4).unwrap().parse::<u32>().unwrap() - 1,
            )
        })
        .collect();

    let mut scores: HashMap<u32, u32> = HashMap::new();

    let mut die = 1;
    let mut player_turn = 0;
    let mut turns = 0;

    while scores.iter().find(|&(_, v)| *v >= 1000).is_none() {
        let position_entry = positions.entry(player_turn + 1).or_default();
        let scores_entry = scores.entry(player_turn + 1).or_default();

        for _ in 0..3 {
            *position_entry = (*position_entry + die) % 10;
            die += 1;
            if die > 100 {
                die = 1;
            }
            turns += 1;
        }
        *scores_entry += *position_entry + 1;

        player_turn += 1;
        player_turn = player_turn % positions.len() as u32;
    }

    println!(
        "Result 1: {}",
        scores.iter().find(|&(_, v)| *v < 1000).unwrap().1 * turns
    );

    let result = solve_2(5, 10, 0, 0, true);

    println!(
        "Result 2: {}",
        if result.0 > result.1 {
            result.0
        } else {
            result.1
        }
    );
}
