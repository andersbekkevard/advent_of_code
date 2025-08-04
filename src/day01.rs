use std::any::type_name;
use std::collections::HashMap;
use std::fs;

fn print_type<T>(_: T) {
    println!("{}", type_name::<T>());
}

fn fill_vecs(path: &String, vec1: &mut Vec<u32>, vec2: &mut Vec<u32>) {
    let contents = fs::read_to_string(path).expect("File-parse-error");
    let mut switch: bool = true;

    for number in contents.split_ascii_whitespace() {
        let mut parsed: u32 = number.parse().expect("Parse error");
        if switch {
            vec1.push(parsed);
        } else {
            vec2.push(parsed);
        }
        switch = !switch;
    }
}

pub fn solve1(path: &String) {
    println!("Solving Day 01...");

    let mut vec1: Vec<u32> = vec![];
    let mut vec2: Vec<u32> = vec![];
    fill_vecs(path, &mut vec1, &mut vec2);

    let mut difference: i32 = 0;
    for i in 0..vec1.len() {
        difference += (vec1[i] as i32 - vec2[i] as i32).abs();
    }
    println!("{difference}");
}
pub fn solve2(path: &String) {
    println!("Solving Day 01...");

    let mut vec1: Vec<u32> = vec![];
    let mut vec2: Vec<u32> = vec![];
    fill_vecs(path, &mut vec1, &mut vec2);

    let mut map: HashMap<u32, u32> = HashMap::new();
    for n in vec2 {
        *map.entry(n).or_insert(0) += 1;
    }
    let mut simularity_score = 0;
    for n in vec1 {
        simularity_score += &n * map.get(&n).unwrap_or(&0);
    }
    println!("{simularity_score}");
}
