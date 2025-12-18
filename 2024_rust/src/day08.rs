use itertools::Itertools;
use std::collections::HashMap;
const EMPTY: char = '.';
//an antinode occurs at any point that is perfectly in line with two antennas of the same frequen

pub fn solve1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let map = grid_to_map(&grid);
    let count = map
        .values()
        .flat_map(find_antinodes)
        .filter(|&(x, y)| x < rows && y < cols)
        .unique()
        .count();
    dbg!(count);
}

fn grid_to_map(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid.first().unwrap().len() {
            let c = grid[i][j];
            if c == EMPTY {
                continue;
            }
            map.entry(c).or_insert_with(Vec::new).push((i, j));
        }
    }
    map
}

fn find_antinodes(antennas: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();
    for ((x1, y1), (x2, y2)) in antennas.iter().copied().tuple_combinations() {
        let delta_x = x2 - x1;
        let delta_y = y2 - y1;
        antinodes.push((x2 + delta_x, y2 + delta_y));

        if let (Some(x), Some(y)) = (x1.checked_sub(delta_x), y1.checked_sub(delta_y)) {
            antinodes.push((x, y));
        }
    }
    antinodes
}

fn print_map(map: HashMap<char, Vec<(usize, usize)>>) {
    for entry in map {
        dbg!(entry);
    }
}
