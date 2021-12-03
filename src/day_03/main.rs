use utils::*;

fn calculate_2(nums: &Vec<Vec<char>>, one: bool) -> u32 {
    let mut filtered_nums = nums.clone();
    for bit in 0..nums[0].len() {
        let one_count = filtered_nums
            .iter()
            .map(|b| b[bit])
            .filter(|b| *b == '1')
            .count();

        let check_bit = if one_count >= (filtered_nums.len() - one_count) {
            if one {
                '1'
            } else {
                '0'
            }
        } else {
            if one {
                '0'
            } else {
                '1'
            }
        };

        filtered_nums = filtered_nums
            .iter()
            .filter(|b| b[bit] == check_bit)
            .map(|b| b.clone())
            .collect();

        if filtered_nums.len() == 1 {
            break;
        }
    }
    let str: String = filtered_nums[0].iter().collect();
    u32::from_str_radix(str.as_str(), 2).unwrap()
}

fn main() {
    let file = read_file("inputs/day_03/input.txt");

    let nums: Vec<Vec<char>> = file.iter().map(|v| v.chars().collect()).collect();

    let num_lenght = nums[0].len();
    let mut gamma = 0;
    (0..num_lenght).for_each(|bit| {
        let one_count = nums.iter().map(|b| b[bit]).filter(|b| *b == '1').count();
        if one_count > (nums.len() - one_count) {
            gamma += 2_u32.pow((num_lenght - 1 - bit) as u32);
        }
    });

    let mask = 2_u32.pow(num_lenght as u32) - 1;
    println!("Result 1: {}", gamma * (!gamma & mask));

    let oxygen = calculate_2(&nums, true);
    let co2 = calculate_2(&nums, false);
    println!("Result 2: {}", oxygen * co2);
}