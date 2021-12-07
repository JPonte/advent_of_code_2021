use utils::*;

fn triangle_n(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn main() {
    let file = read_file("inputs/day_07/input.txt");

    let crabs: Vec<i32> = file[0].split(",").map(|c| c.parse().unwrap()).collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let result: (i32, i32) = (*min..*max)
        .map(|i| (i, crabs.iter().map(|c| (*c - i).abs()).sum()))
        .min_by_key(|(_, sum)| *sum)
        .unwrap();

    println!("Result 1: {}", result.1);

    let result2: (i32, i32) = (*min..*max)
        .map(|i| (i, crabs.iter().map(|c| triangle_n((*c - i).abs())).sum()))
        .min_by_key(|(_, sum)| *sum)
        .unwrap();

    println!("Result 2: {}", result2.1);
}
