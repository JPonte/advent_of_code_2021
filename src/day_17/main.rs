use utils::*;

fn limit_x(vec: i32) -> i32 {
    ((vec * vec) + vec) / 2
}

fn simulate(
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
    start_vel_x: i32,
    start_vel_y: i32,
) -> (i32, bool) {
    let mut max_y = 0;
    let (mut x, mut y) = (0, 0);
    let (mut vec_x, mut vec_y) = (start_vel_x, start_vel_y);
    while x < end_x && y > start_y {
        x += vec_x;
        y += vec_y;
        max_y = max_y.max(y);
        vec_x = (vec_x - 1).max(0);
        vec_y -= 1;
        if x >= start_x && x <= end_x && y >= start_y && y <= end_y {
            return (max_y, true);
        }
    }
    (max_y, false)
}

fn main() {
    let file = read_file("inputs/day_17/input.txt");
    let line = file.get(0).unwrap().to_string();
    let start = "target area: ";

    let ranges_vec: Vec<(i32, i32)> = line[start.len()..line.len()]
        .split(", ")
        .map(|e| {
            let range: Vec<i32> = e[2..e.len()]
                .split("..")
                .map(|r| r.parse().unwrap())
                .collect();
            (range[0], range[1])
        })
        .collect();

    let ((start_x, end_x), (start_y, end_y)) = (ranges_vec[0], ranges_vec[1]);

    let possible_vec_x = (0..=end_x)
        .filter(|x| limit_x(*x) >= start_x)
        .collect::<Vec<i32>>();
        
    let vel_y = start_y.abs() - 1;
    let max_y = possible_vec_x
        .iter()
        .map(|vel_x| simulate(start_x, end_x, start_y, end_y, *vel_x, vel_y).0)
        .max()
        .unwrap();

    println!("Result 1: {}", max_y);

    let possible_vec_y = (start_y..=start_y.abs()).collect::<Vec<i32>>();

    let all = possible_vec_x
        .iter()
        .flat_map(|x| {
            possible_vec_y
                .iter()
                .map(move |y| (*x, *y))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<(i32, i32)>>();

    let count = all
        .iter()
        .filter(|(vel_x, vel_y)| simulate(start_x, end_x, start_y, end_y, *vel_x, *vel_y).1)
        .count();
    println!("Result 2: {}", count);
}
