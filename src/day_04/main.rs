use utils::*;

fn score_board(board: &Vec<Vec<&str>>, sequence: &Vec<&str>) -> Option<(u32, u32)> {
    let mut round = 0;

    let rotated_board: Vec<Vec<&str>> = board[0]
        .iter()
        .enumerate()
        .map(|(i, _)| board.iter().map(|line| line[i]).collect())
        .collect();

    while round < sequence.len() {
        let nums = sequence[0..round].to_vec();

        let horizontal = board
            .iter()
            .filter(|l| nums.iter().filter(|n| l.contains(*n)).count() == l.len())
            .count()
            > 0;
        let vertical = rotated_board
            .iter()
            .filter(|l| nums.iter().filter(|n| l.contains(*n)).count() == l.len())
            .count()
            > 0;

        if horizontal || vertical {
            let unmarked: u32 = board
                .iter()
                .flat_map(|l| l)
                .filter(|c| !nums.contains(c))
                .map(|c| c.parse::<u32>().unwrap())
                .sum();
            let last: u32 = sequence[round - 1].parse().unwrap();

            return Some((round as u32, unmarked * last));
        }
        round += 1;
    }

    None
}

fn main() {
    let file = read_file("inputs/day_04/input.txt");

    let sequence: Vec<&str> = file[0].split(",").collect();

    let boards: Vec<Vec<Vec<&str>>> = file.iter().skip(2).fold(Vec::new(), |mut acc, l| {
        if l == "" {
            acc.push(Vec::new());
        } else {
            if acc.is_empty() {
                acc.push(Vec::new());
            }
            acc.last_mut()
                .unwrap()
                .push(l.split(" ").filter(|n| *n != "").collect());
        }

        acc
    });

    let winner = boards
        .iter()
        .flat_map(|board| score_board(&board, &sequence))
        .min_by_key(|res| res.0);

    println!("Result 1: {}", winner.unwrap().1);

    let loser = boards
        .iter()
        .flat_map(|board| score_board(&board, &sequence))
        .max_by_key(|res| res.0);

    println!("Result 2: {}", loser.unwrap().1);
}
