use std::collections::HashMap;

use utils::*;

fn main() {
    let file = read_file("inputs/day_06/input.txt");

    let ages: Vec<u64> = file[0].split(",").map(|a| a.parse().unwrap()).collect();

    let mut fish_map: HashMap<u64, u64> = HashMap::new();
    ages.iter().for_each(|a| {
      fish_map.entry(*a).and_modify(|c| *c += 1).or_insert(1);
    });

    for day in 0..256 {
      let mut new_fish: HashMap<u64, u64> = HashMap::new();
      fish_map.iter().for_each(|(&age, &count)| {
         match age {
            0 => {
              new_fish.entry(6).and_modify(|c| *c += count).or_insert(count);
              new_fish.entry(8).and_modify(|c| *c += count).or_insert(count);
            },
            n => {
              new_fish.entry(n - 1).and_modify(|c| *c += count).or_insert(count);
            }
         }
      });
      fish_map = new_fish;
      if day == 79 {
         println!("Result 1: {}", fish_map.iter().map(|(_, c)| c).sum::<u64>());     
      }
    }

    println!("Result 2: {}", fish_map.iter().map(|(_, c)| c).sum::<u64>());
}
