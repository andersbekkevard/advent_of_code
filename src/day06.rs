use std::fs;
const X: char = 'X';
const GUARD: char = '^';
const BLOCK: char = '#';
const BIG: usize = 1_000_000;
/*
 *Current status:
 *The problem lies in the incrementation of usize, who get parsed to i32 and manually cast
 *back as BIG. If you fix this, the code should work (i think). Sexy rabbit/turtle algo
 * In addition, we need to have a shared reference for modification between rabbit/turtle
 * Right now they both check the original, unmodified vec
 */

#[derive(Debug, Clone, Copy)]
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
        let status: MoveCheck = check_move(&map, i, j, dir);
        if status == MoveCheck::Finished {
            break;
        }
        if status == MoveCheck::Blocked {
            dir = dir.rotate();
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
    let (mut start_row, mut start_col) = find_guard(&map).unwrap();
    let dir = Direction::North;
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if start_row == i && start_col == j {
                continue;
            }

            if has_loop(&map, i, j, dir) {
                count += 1;
            }
        }
    }
    dbg!(count);
}

fn create_vec(map_to_check: &[Vec<char>], i: usize, j: usize) -> Vec<Vec<char>> {
    let mut vec = map_to_check.clone();
    vec[i][j] = BLOCK;
    vec
}

fn has_loop(map_to_check: &[Vec<char>], i: usize, j: usize, dir: Direction) -> bool {
    let mut map = map_to_check.clone();
    let mut i_fast = i;
    let mut j_fast = j;
    let mut i_slow = i;
    let mut j_slow = j;
    let mut dir_fast = dir;
    let mut dir_slow = dir;
    let mut switch = false;

    loop {
        let status_fast: MoveCheck = check_move(map.as_slice(), i, j, dir);

        if status_fast == MoveCheck::Finished {
            break;
        }
        if status_fast == MoveCheck::Blocked {
            dir_fast = dir_fast.rotate();
        }
        (i_fast, j_fast) = move_direction(&mut map, i_fast, j_fast, dir_fast);

        if switch {
            let status_slow: MoveCheck = check_move(&map, i, j, dir);
            if status_slow == MoveCheck::Finished {
                break;
            }
            if status_slow == MoveCheck::Blocked {
                dir_slow = dir_slow.rotate();
            }
            (i_slow, j_slow) = move_direction(&mut map, i_slow, j_slow, dir_slow);
        }
        if i_fast == i_slow && j_fast == j_slow {
            return true;
        }
        switch = !switch;
    }
    false
}

fn check_move(matrix: &Vec<Vec<char>>, i: usize, j: usize, dir: Direction) -> MoveCheck {
    let (row_index, col_index) = increment(i, j, dir).unwrap();
    if !in_bounds(matrix, row_index, col_index) {
        return MoveCheck::Finished;
    } else if matrix[row_index][col_index] == BLOCK {
        return MoveCheck::Blocked;
    }
    MoveCheck::Good
}

fn increment_old(i: usize, j: usize, dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::North => Some((i - 1, j)),
        Direction::South => Some((i + 1, j)),
        Direction::East => Some((i, j + 1)),
        Direction::West => Some((i, j - 1)),
    }
}

fn increment_32(i: i32, j: i32, dir: Direction) -> Option<(i32, i32)> {
    match dir {
        Direction::North => Some((i - 1, j)),
        Direction::South => Some((i + 1, j)),
        Direction::East => Some((i, j + 1)),
        Direction::West => Some((i, j - 1)),
    }
}
fn increment(i: usize, j: usize, dir: Direction) -> Option<(usize, usize)> {
    let (i32, j32) = increment_32(i as i32, j as i32, dir).unwrap();

    if i32 < 0 {
        let i = BIG;
    } else {
        let i = i32 as usize;
    }
    if j32 < 0 {
        let j = BIG;
    } else {
        let j = j32 as usize;
    }
    Some((i, j))
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
}
