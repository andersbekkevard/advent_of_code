use std::fs;

pub fn solve1(path: String) {
    let contents = fs::read_to_string(path).expect("Parse error");
    let pattern = "XMAS";

    let matrix: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    let permutations = create_permutations(matrix);
    debug(&permutations);
    let mut count: usize = permutations
        .iter()
        .map(|s| count_both_ways(s, pattern))
        .sum();
    println!("{count}");
}
fn count_in_string(s: &str, pattern: &str) -> usize {
    s.match_indices(pattern).count()
}
fn count_both_ways(s: &str, pattern: &str) -> usize {
    let reversed: String = String::from(s).chars().rev().collect();
    count_in_string(s, pattern) + count_in_string(reversed.as_str(), pattern)
}

fn create_permutations(matrix: Vec<Vec<char>>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let row_len = matrix.get(0).unwrap().len();
    let col_len = matrix.len();

    println!("Permuting horizontally");
    // horizontal
    for r in matrix.clone() {
        out.push(vec_to_string(r.clone()));
    }

    println!("Permuting vertically");
    // vertical
    for i in 0..row_len {
        let mut col: Vec<char> = Vec::new();
        for r in matrix.clone() {
            col.push(*r.get(i).unwrap());
        }
        out.push(vec_to_string(col));
    }

    println!("Permuting diagonally - left to right");
    // diagonal - left to right
    let len: i32 = row_len as i32;
    for row in -(len - 1)..len {
        let mut i = row;
        let mut j = 0;
        let mut diag: Vec<char> = Vec::new();
        while (i < len && i >= 0) || (j < len && j >= 0) {
            if i < len && i >= 0 && j < len && j >= 0 {
                diag.push(
                    matrix
                        .get(i as usize)
                        .unwrap()
                        .get(j as usize)
                        .unwrap()
                        .clone(),
                );
            }
            i += 1;
            j += 1;
        }
        println!("Row {}: {:?} (len: {})", row, diag.clone(), diag.len());
        out.push(vec_to_string(diag));
    }

    println!("Permuting diagonally - right to left");
    // diagonal - right to left
    for row in -(len - 1)..len {
        let mut i = row;
        let mut j = len - 1;
        let mut diag: Vec<char> = Vec::new();
        while (i < len && i >= 0) || (j < len && j >= 0) {
            if i < len && i >= 0 && j < len && j >= 0 {
                diag.push(
                    matrix
                        .get(i as usize)
                        .unwrap()
                        .get(j as usize)
                        .unwrap()
                        .clone(),
                );
            }
            i += 1;
            j -= 1;
        }
        println!("Row {}: {:?} (len: {})", row, diag.clone(), diag.len());
        out.push(vec_to_string(diag));
    }

    out
}

fn vec_to_string(vec: Vec<char>) -> String {
    vec.into_iter().collect()
}

fn debug(permutations: &Vec<String>) {
    let pattern = "XMAS";
    let mut x = 0;
    for s in permutations.clone() {
        println!("{} with count {}", s, count_both_ways(&s, &pattern));
        x += 1;
        if x % 10 == 0 {
            println!();
        }
    }
}

pub fn solve2(path: String) {
    let contents = fs::read_to_string(path).expect("Parse error");

    let matrix: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    let mut count = 0;

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if check_around_indexes(&i, &j, &matrix) {
                println!("Found match at position ({}, {})", i, j);
                count += 1;
            }
        }
    }

    println!("{count}");
}

fn check_around_indexes(i: &usize, j: &usize, matrix: &Vec<Vec<char>>) -> bool {
    if matrix[*i][*j] != 'A' {
        return false;
    }
    // diag: left to right
    let diag_l = get_three_chars(matrix, *i, *j, -1, -1, 1, 1);

    // diag: right to left
    let diag_r = get_three_chars(matrix, *i, *j, -1, 1, 1, -1);

    println!("diag_l={:?}, diag_r{:?}", &diag_l, &diag_r);
    let result = check_vec(&diag_l) && check_vec(&diag_r);
    // check_vec(&horizontal) + check_vec(&vertical) +
    result
}
fn check_vec(vec: &Vec<char>) -> bool {
    let result = (vec[0] == 'M' && vec[1] == 'A' && vec[2] == 'S')
        || (vec[2] == 'M' && vec[1] == 'A' && vec[0] == 'S');

    result
}

fn get_or_default(matrix: &Vec<Vec<char>>, i: &usize, j: &usize, default: char) -> char {
    match matrix.get(*i) {
        Some(vec) => {
            return *vec.get(*j).unwrap_or(&default);
        }
        None => {
            return default;
        }
    }
}

fn get_three_chars(
    matrix: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    di1: i32,
    dj1: i32,
    di2: i32,
    dj2: i32,
) -> Vec<char> {
    let mut result = Vec::new();
    let len = matrix.len() as i32;
    let i_1 = i as i32 + di1;
    let j_1 = j as i32 + dj1;
    let i_2 = i as i32 + di2;
    let j_2 = j as i32 + dj2;

    if valid_index(i_1, j_1, &matrix) {
        result.push(matrix[i_1 as usize][j_1 as usize]);
    } else {
        result.push('0');
    }

    result.push(matrix[i][j]);

    if valid_index(i_2, j_2, &matrix) {
        result.push(matrix[i_2 as usize][j_2 as usize]);
    } else {
        result.push('0');
    }
    result
}

fn valid_index<T>(i: i32, j: i32, matrix: &Vec<Vec<T>>) -> bool {
    let rows = matrix.len() as i32;
    let cols = matrix.get(0).unwrap().len() as i32;
    let result = (0 <= i && i < rows) && (0 <= j && j < cols);
    result
}
