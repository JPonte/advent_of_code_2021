use utils::*;

fn main() {
    let file = read_file("inputs/day_01/input.txt");

    let depths: Vec<u32> = file.iter().map(|l| l.parse().unwrap()).collect();

    let result: u32 = depths
        .iter()
        .zip(depths.iter().skip(1))
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();

    println!("First: {}", result);

    let windowed_depths: Vec<u32> = depths
        .iter()
        .take(depths.len() - 2)
        .enumerate()
        .map(|(i, a)| *a + depths[i + 1] + depths[i + 2])
        .collect();

    let result_2: u32 = windowed_depths
        .iter()
        .zip(windowed_depths.iter().skip(1))
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();

    println!("Second: {}", result_2);
}
