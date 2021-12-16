use utils::*;

fn process(chars: &Vec<char>, from: usize, version_acc: &mut u64) -> (u64, usize) {
    let mut i = from;
    let version =
        u8::from_str_radix(chars[i..i + 3].iter().collect::<String>().as_str(), 2).unwrap();
    *version_acc += version as u64;
    let type_id =
        u8::from_str_radix(chars[i + 3..i + 6].iter().collect::<String>().as_str(), 2).unwrap();
    i += 6;

    if type_id == 4 {
        let mut literal_buff = Vec::new();
        loop {
            let flag = chars[i].to_digit(2).unwrap() == 1;
            let literal =
                u64::from_str_radix(chars[i + 1..i + 5].iter().collect::<String>().as_str(), 2)
                    .unwrap();
            literal_buff.push(literal);
            i += 5;
            if !flag {
                let literal = literal_buff
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (i, l)| acc + l * 2_u64.pow((i * 4) as u32));
                return (literal, i);
            }
        }
    } else {
        let flag = chars[i].to_digit(2).unwrap() == 1;
        i += 1;

        let mut nums = Vec::new();

        if flag {
            let len =
                usize::from_str_radix(chars[i..i + 11].iter().collect::<String>().as_str(), 2)
                    .unwrap();
            i += 11;
            for _ in 0..len {
                let (num, next_i) = process(chars, i, version_acc);
                nums.push(num);
                i = next_i;
            }
        } else {
            let len =
                usize::from_str_radix(chars[i..i + 15].iter().collect::<String>().as_str(), 2)
                    .unwrap();
            i += 15;
            let start_i = i;
            while (i - start_i) < len {
                let (num, next_i) = process(chars, i, version_acc);
                nums.push(num);
                i = next_i;
            }
        }
        match type_id {
            0 => (nums.iter().sum(), i),
            1 => (nums.iter().product(), i),
            2 => (*nums.iter().min().unwrap(), i),
            3 => (*nums.iter().max().unwrap(), i),
            5 => ((nums[0] > nums[1]) as u64, i),
            6 => ((nums[0] < nums[1]) as u64, i),
            7 => ((nums[0] == nums[1]) as u64, i),
            _ => panic!("oops"),
        }
    }
}

fn main() {
    let file = read_file("inputs/day_16/input.txt");
    for line in file {
        let chars: Vec<char> = line
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .flat_map(|x| format!("{:04b}", x).chars().collect::<Vec<char>>())
            .collect();
        let mut version_sum = 0;
        let (result, _) = process(&chars, 0, &mut version_sum);
        println!("Result 1: {}", version_sum);
        println!("Result 2: {}", result);
    }
}
