use std::cmp::Ordering;
use std::fs;

pub fn solve1(path: &str) {
    let contents = fs::read_to_string(path).expect("Parse error");
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let mut rules: Vec<Pair> = parts[0].split_whitespace().map(Pair::from_str).collect();

    let updates: Vec<Vec<i32>> = parts[1]
        .split_whitespace()
        .map(|l| l.split(',').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    for (rule, upd) in rules.iter_mut().zip(&updates) {
        println!("{upd:?} valid: {}", rule.verify(upd));
    }
}

struct Pair {
    comparator: Box<dyn FnMut(&i32, &i32) -> Ordering>,
}

impl Pair {
    fn new(first: i32, second: i32) -> Self {
        Self {
            comparator: Box::new(move |a, b| {
                if *a == second && *b == first {
                    Ordering::Greater // reverse pair
                } else if *a == first && *b == second {
                    Ordering::Less // normal pair
                } else {
                    Ordering::Equal
                }
            }),
        }
    }

    fn from_str(s: &str) -> Self {
        let (first, second) = s
            .split_once('|')
            .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
            .unwrap();
        Self::new(first, second)
    }

    fn verify(&mut self, update: &[i32]) -> bool {
        update.is_sorted_by(|a, b| (self.comparator)(a, b) != Ordering::Greater)
    }
}
