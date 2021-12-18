use utils::*;

#[derive(Debug, Clone)]
enum Node {
    Num(u32),
    Open,
    Close,
}

pub trait NodeVec {
    fn get_num(&self, i: usize) -> Option<u32>;
    fn add_num(&mut self, i: usize, num: u32);
}

impl NodeVec for Vec<Node> {
    fn get_num(&self, i: usize) -> Option<u32> {
        match self.get(i) {
            Some(Node::Num(x)) => Some(*x),
            _ => None,
        }
    }

    fn add_num(&mut self, i: usize, num: u32) {
        if let Some(current) = self.get_num(i) {
            let _ = std::mem::replace(&mut self[i], Node::Num(current + num));
        }
    }
}

fn parse(chars: &Vec<char>) -> Vec<Node> {
    chars
        .iter()
        .flat_map(|c| match *c {
            ']' => Some(Node::Close),
            '[' => Some(Node::Open),
            x if x.is_digit(10) => Some(Node::Num(x.to_digit(10).unwrap())),
            _ => None,
        })
        .collect()
}

fn find_deep_node(nodes: &Vec<Node>) -> Option<usize> {
    let mut depth = 0;
    let mut last_is_num = false;
    for (i, node) in nodes.iter().enumerate() {
        match node {
            Node::Open => {
                depth += 1;
                last_is_num = false;
            }
            Node::Close => {
                depth -= 1;
                last_is_num = false;
            }
            Node::Num(_) if depth > 4 => {
                if last_is_num {
                    return Some(i - 1);
                } else {
                    last_is_num = true;
                }
            }
            _ => {}
        }
    }
    None
}

fn find_high_node(nodes: &Vec<Node>) -> Option<usize> {
    for (i, node) in nodes.iter().enumerate() {
        match node {
            Node::Num(x) if *x > 9 => return Some(i),
            _ => {}
        }
    }
    None
}

fn concat_nodes(nodes1: &Vec<Node>, nodes2: &Vec<Node>) -> Vec<Node> {
    [
        vec![Node::Open],
        nodes1.clone(),
        nodes2.clone(),
        vec![Node::Close],
    ]
    .concat()
}

fn magnitude(nodes: &mut Vec<Node>, left: bool) -> u64 {
    let node = nodes.pop();
    if let Some(node) = node {
        match node {
            Node::Open => {
                let int = 3 * magnitude(nodes, left) + 2 * magnitude(nodes, left);
                int
            }
            Node::Num(x) => x as u64,
            Node::Close => magnitude(nodes, left),
        }
    } else {
        0
    }
}

fn reduce(nodes: &mut Vec<Node>) {
    loop {
        let (deep_node, high_node) = (find_deep_node(&nodes), find_high_node(&nodes));
        if let Some(deep_node) = deep_node {
            let deep_left = nodes.get_num(deep_node).unwrap();
            let deep_right = nodes.get_num(deep_node + 1).unwrap();
            if let Some((left_i, _)) = nodes.iter().enumerate().rev().find(|(i, node)| {
                *i < deep_node
                    && match node {
                        Node::Num(_) => true,
                        _ => false,
                    }
            }) {
                nodes.add_num(left_i, deep_left);
            }
            if let Some((right_i, _)) = nodes.iter().enumerate().find(|(i, node)| {
                *i > deep_node + 1
                    && match node {
                        Node::Num(_) => true,
                        _ => false,
                    }
            }) {
                nodes.add_num(right_i, deep_right);
            }
            nodes.remove(deep_node - 1);
            nodes.remove(deep_node - 1);
            nodes.remove(deep_node - 1);
            nodes.remove(deep_node - 1);
            nodes.insert(deep_node - 1, Node::Num(0));
        } else if let Some(high_node) = high_node {
            let high_value = nodes.get_num(high_node).unwrap();
            let left = (high_value as f32 / 2f32).floor() as u32;
            let right = (high_value as f32 / 2f32).ceil() as u32;
            nodes.remove(high_node);
            for n in vec![Node::Close, Node::Num(right), Node::Num(left), Node::Open] {
                nodes.insert(high_node, n);
            }
        } else {
            break;
        }
    }
}

fn main() {
    let file = read_file("inputs/day_18/input.txt");
    let parsed: Vec<Vec<Node>> = file
        .iter()
        .map(|line| parse(&line.chars().collect()))
        .collect();

    let mut final_list = parsed.iter().fold(Vec::new(), |acc, nodes| {
        let mut added = if acc.is_empty() {
            nodes.clone()
        } else {
            concat_nodes(&acc, nodes)
        };
        reduce(&mut added);

        added
    });

    final_list.reverse();
    println!("Result 1: {}", magnitude(&mut final_list, true));

    let max = parsed
        .iter()
        .enumerate()
        .flat_map(|(i, nodes)| {
            parsed
                .iter()
                .enumerate()
                .filter(move |(j, _)| i != *j)
                .map(move |(j, other)| {
                    let mut pair = concat_nodes(nodes, other);
                    reduce(&mut pair);
                    pair.reverse();
                    let m = magnitude(&mut pair, true);
                    (i, j, m)
                })
                .collect::<Vec<(usize, usize, u64)>>()
        })
        .max_by_key(|(_, _, x)| *x)
        .unwrap()
        .2;

    println!("Result 2: {}", max);
}
