use std::collections::{HashMap, HashSet};

use utils::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Beacon(i32, i32, i32);

impl std::ops::Sub<Beacon> for Beacon {
    type Output = Beacon;

    fn sub(self, rhs: Beacon) -> Beacon {
        Beacon(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Add<Beacon> for Beacon {
    type Output = Beacon;

    fn add(self, rhs: Beacon) -> Beacon {
        Beacon(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Beacon {
    fn rotate_x(&self, amount: usize) -> Beacon {
        let ops = vec![
            Beacon(self.0, self.1, self.2),
            Beacon(self.0, self.2, -self.1),
            Beacon(self.0, -self.2, self.1),
            Beacon(self.0, -self.1, -self.2),
        ];
        ops[amount]
    }
    fn rotate_y(&self, amount: usize) -> Beacon {
        let ops = vec![
            Beacon(self.0, self.1, self.2),
            Beacon(self.2, self.1, -self.0),
            Beacon(-self.2, self.1, self.0),
            Beacon(-self.0, self.1, -self.2),
        ];
        ops[amount]
    }
    fn rotate_z(&self, amount: usize) -> Beacon {
        let ops = vec![
            Beacon(self.0, self.1, self.2),
            Beacon(self.1, -self.0, self.2),
            Beacon(-self.1, self.0, self.2),
            Beacon(-self.0, -self.1, self.2),
        ];
        ops[amount]
    }
}

fn matching_scans(scan0: &Vec<Beacon>, scan1: &Vec<Beacon>) -> Option<(Beacon, Vec<Beacon>)> {
    let pivoted0 = scan0
        .iter()
        .map(|pivot| {
            (
                *pivot,
                scan0
                    .iter()
                    .map(move |b| *b - *pivot)
                    .collect::<HashSet<Beacon>>(),
            )
        })
        .collect::<Vec<(Beacon, HashSet<Beacon>)>>();

    let orientations: Vec<(usize, usize, usize)> = (0..4)
        .flat_map(|x| {
            (0..4)
                .flat_map(move |y| {
                    (0..4)
                        .map(move |z| (x, y, z))
                        .collect::<Vec<(usize, usize, usize)>>()
                })
                .collect::<Vec<(usize, usize, usize)>>()
        })
        .collect();
    for (rx, ry, rz) in orientations {
        let rotated: Vec<Beacon> = scan1
            .iter()
            .map(|b| b.rotate_x(rx).rotate_y(ry).rotate_z(rz))
            .collect();

        let pivoted1 = rotated
            .iter()
            .map(|pivot| {
                (
                    *pivot,
                    rotated
                        .iter()
                        .map(move |b| *b - *pivot)
                        .collect::<HashSet<Beacon>>(),
                )
            })
            .collect::<Vec<(Beacon, HashSet<Beacon>)>>();

        for (p0, b0) in pivoted0.iter() {
            for (p1, b1) in pivoted1.iter() {
                if b1.intersection(b0).count() >= 12 {
                    return Some((*p0 - *p1, rotated));
                }
            }
        }
    }

    None
}

fn get_beacons_positions(
    scans: &mut Vec<Vec<Beacon>>,
    base_scanner: usize,
    scanner_positions: &mut HashMap<usize, Beacon>,
) {
    let scan0 = &scans[base_scanner];
    let base_position = *scanner_positions.get(&base_scanner).unwrap();

    let mut to_explore = Vec::new();
    let new_scans: Vec<Vec<Beacon>> = scans
        .iter()
        .enumerate()
        .map(|(j, scan1)| {
            if base_scanner == j {
                scan1.clone()
            } else if let Some((pos, fixed_scan)) = matching_scans(&scan0, scan1) {
                if !scanner_positions.contains_key(&j) {
                    scanner_positions.insert(j, base_position + pos);
                    to_explore.push(j);
                }
                fixed_scan
            } else {
                scan1.clone()
            }
        })
        .collect();

    *scans = new_scans;
    for te in to_explore {
        get_beacons_positions(scans, te, scanner_positions);
    }
}

fn main() {
    let file = read_file("inputs/day_19/input.txt");

    let mut scans = Vec::new();
    let mut beacons = Vec::new();
    for line in file {
        if line.starts_with("--") {
            continue;
        }
        if line == "" {
            scans.push(beacons.clone());
            beacons.clear();
            continue;
        }
        let coords: Vec<i32> = line
            .split(",")
            .map(|coord| coord.parse::<i32>().unwrap())
            .collect();
        beacons.push(Beacon(coords[0], coords[1], coords[2]));
    }
    if beacons.len() > 0 {
        scans.push(beacons.clone());
        beacons.clear();
    }

    let mut scanner_positions = HashMap::new();
    scanner_positions.insert(0, Beacon(0, 0, 0));

    get_beacons_positions(&mut scans, 0, &mut scanner_positions);

    let uniques = scans
        .iter()
        .enumerate()
        .flat_map(|(i, scan)| {
            if let Some(scan_pos) = scanner_positions.get(&i) {
                scan.iter()
                    .map(move |b| *scan_pos + *b)
                    .collect::<HashSet<Beacon>>()
            } else {
                HashSet::new()
            }
        })
        .collect::<HashSet<Beacon>>();

    println!("Result 1: {}", uniques.len());

    let max = scanner_positions
        .iter()
        .flat_map(|(_, s1)| {
            scanner_positions.iter().map(|(_, s2)| {
                let d = *s1 - *s2;
                d.0.abs() + d.1.abs() + d.2.abs()
            })
        })
        .max()
        .unwrap();

    println!("Result 2: {}", max);
}
