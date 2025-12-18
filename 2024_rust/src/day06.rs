use std::fs;
const X: char = 'X';
const GUARD: char = '^';
const BLOCK: char = '#';
const ADDED_BLOCK: char = 'O';

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq)]
enum MoveCheck {
    Good,
    Blocked,
    Finished,
}

pub fn solve1(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let mut map: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    let (mut i, mut j) = find_guard(&map).unwrap();
    let mut dir = Direction::North;

    loop {
        let mut status: MoveCheck = check_move(&map, i, j, dir);
        if status == MoveCheck::Finished {
            break;
        }
        while status == MoveCheck::Blocked {
            dir = dir.rotate();
            status = check_move(&map, i, j, dir);
        }
        assert!(check_move(&map, i, j, dir) != MoveCheck::Blocked);
        (i, j) = move_direction(&mut map, i, j, dir);
    }
    let result = count(&map);
}

pub fn solve2(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let map: Vec<Vec<char>> = contents
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    let (start_row, start_col) = find_guard(&map).unwrap();
    let dir = Direction::North;
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if start_row == i && start_col == j {
                continue;
            }

            if has_loop(&map, i, j, start_row, start_col, dir) {
                println!("i={}, j={}\n\n\n\n\n", i, j);
                count += 1;
            }
        }
    }
    dbg!(count);
}

fn create_vec(map_to_check: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<Vec<char>> {
    let mut vec = map_to_check.clone();
    vec[i][j] = ADDED_BLOCK;
    vec
}

fn has_loop(
    map_to_check: &Vec<Vec<char>>,
    block_row: usize,
    block_col: usize,
    start_row: usize,
    start_col: usize,
    dir: Direction,
) -> bool {
    let mut map = create_vec(map_to_check, block_row, block_col);
    let (mut i_fast, mut j_fast, mut i_slow, mut j_slow) =
        (start_row, start_col, start_row, start_col);
    let (mut dir_fast, mut dir_slow) = (dir, dir);
    let mut switch = false;

    loop {
        let mut status_fast: MoveCheck = check_move(&map, i_fast, j_fast, dir_fast);

        if status_fast == MoveCheck::Finished {
            break;
        }
        while status_fast == MoveCheck::Blocked {
            dir_fast = dir_fast.rotate();
            status_fast = check_move(&map, i_fast, j_fast, dir_fast);
        }
        (i_fast, j_fast) = move_direction(&mut map, i_fast, j_fast, dir_fast);

        if switch {
            let mut status_slow: MoveCheck = check_move(&map, i_slow, j_slow, dir_slow);
            if status_slow == MoveCheck::Finished {
                break;
            }
            while status_slow == MoveCheck::Blocked {
                dir_slow = dir_slow.rotate();
                status_slow = check_move(&map, i_slow, j_slow, dir_slow);
            }
            (i_slow, j_slow) = move_direction(&mut map, i_slow, j_slow, dir_slow);
        }
        if i_fast == i_slow && j_fast == j_slow && dir_fast == dir_slow {
            print_map(&map);
            return true;
        }
        switch = !switch;
    }
    false
}

fn check_move(matrix: &Vec<Vec<char>>, i: usize, j: usize, dir: Direction) -> MoveCheck {
    match increment(i, j, dir) {
        Some((row_index, col_index)) => {
            if !in_bounds(matrix, row_index, col_index) {
                return MoveCheck::Finished;
            } else if matrix[row_index][col_index] == BLOCK
                || matrix[row_index][col_index] == ADDED_BLOCK
            {
                return MoveCheck::Blocked;
            }
            MoveCheck::Good
        }
        None => MoveCheck::Finished,
    }
}

fn increment_32(i: i32, j: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (i - 1, j),
        Direction::South => (i + 1, j),
        Direction::East => (i, j + 1),
        Direction::West => (i, j - 1),
    }
}
fn increment(i: usize, j: usize, dir: Direction) -> Option<(usize, usize)> {
    let (i32, j32) = increment_32(i as i32, j as i32, dir);

    if i32 < 0 || j32 < 0 {
        return None;
    }
    Some((i32 as usize, j32 as usize))
}

/// This method is dumb, simply moves and replaces
fn move_direction(
    matrix: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    dir: Direction,
) -> (usize, usize) {
    let (row_index, col_index) = increment(i, j, dir).unwrap();
    matrix[row_index][col_index] = GUARD;
    matrix[i][j] = X;
    (row_index, col_index)
}

fn find_guard(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == GUARD {
                return Some((i, j));
            }
        }
    }
    None
}

fn count(map: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == X || map[i][j] == GUARD {
                count += 1;
            }
        }
    }
    count
}

fn in_bounds(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    row < matrix.len() && col < matrix[row].len()
}

fn print_map(map: &Vec<Vec<char>>) {
    for vec in map {
        for c in vec {
            print!("{c}");
        }
        println!();
    }
    println!();
    println!();
}
