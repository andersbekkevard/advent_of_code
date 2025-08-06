use std::{cmp::Ordering, fs};

pub fn solve1(path: &str) {
    let contents = fs::read_to_string(path).expect("Parse error");
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let pairs: Vec<Pair> = parts[0].split_whitespace().map(Pair::from_str).collect();

    let updates: Vec<Vec<i32>> = parts[1]
        .split_whitespace()
        .map(|l| l.split(',').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    let result: i32 = updates
        .iter()
        .filter(|u| verify_all(u, &pairs))
        .map(fetch_middle)
        .sum();
    dbg!(result);
}
pub fn solve2(path: &str) {
    let contents = fs::read_to_string(path).expect("Parse error");
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let pairs: Vec<Pair> = parts[0].split_whitespace().map(Pair::from_str).collect();

    let mut updates: Vec<Vec<i32>> = parts[1]
        .split_whitespace()
        .map(|l| l.split(',').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
    let mut invalid: Vec<&mut Vec<i32>> = updates
        .iter_mut()
        .filter(|u| !verify_all(u, &pairs))
        .collect();
    for update in &mut invalid {
        sort_update(update, &pairs);
    }
    let result: i32 = invalid.iter().map(|u| fetch_middle(u)).sum();
    dbg!(result);
}

fn sort_update(update: &mut Vec<i32>, pairs: &Vec<Pair>) {
    update.sort_by(|a, b| {
        for p in pairs {
            if p.first == *a && p.second == *b {
                return Ordering::Less;
            } else if p.first == *b && p.second == *a {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
}

#[derive(Debug)]
struct Pair {
    first: i32,
    second: i32,
}

impl Pair {
    fn new(first: i32, second: i32) -> Self {
        Self { first, second }
    }
    fn from_str(s: &str) -> Self {
        let (first, second) = s
            .split_once('|')
            .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
            .unwrap();
        Self::new(first, second)
    }
    fn verify(&self, update: &Vec<i32>) -> bool {
        let first_pos = update.iter().position(|n| *n == self.first);
        let second_pos = update.iter().position(|n| *n == self.second);
        if let (Some(first_idx), Some(second_idx)) = (first_pos, second_pos) {
            if first_idx > second_idx {
                return false;
            }
        }
        true
    }
}
fn verify_all(update: &Vec<i32>, pairs: &[Pair]) -> bool {
    pairs.iter().all(|p| p.verify(update))
}

fn fetch_middle(update: &Vec<i32>) -> i32 {
    let i: usize = update.len() / 2;
    update[i]
}
