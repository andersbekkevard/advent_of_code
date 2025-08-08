pub fn solve1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines: Vec<(u32, Vec<u32>)> = input.lines().map(parse_line).collect();
    let mut sum = 0;
    for (result, nums) in lines {
        let mut results: Vec<u32> = Vec::new();
        calculate_from(&nums, &mut results, 0, 0, Operator::ADD);
        calculate_from(&nums, &mut results, 0, 0, Operator::MULTIPLY);
        if results.contains(&result) {
            sum += result;
        }
    }
    dbg!(sum);
}

enum Operator {
    MULTIPLY,
    ADD,
}

fn parse_line(line: &str) -> (u32, Vec<u32>) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let result: u32 = parts[0].trim_end_matches(":").parse().unwrap();
    let nums: Vec<u32> = parts[1..].iter().map(|s| s.parse().unwrap()).collect();
    return (result, nums);
}

fn can_be_fixed(result: u32, nums: &Vec<u32>) {}

fn calculate_from(
    nums: &Vec<u32>,
    results: &mut Vec<u32>,
    current_result: u32,
    index: usize,
    operator: Operator,
) {
    let next_result = perform_operation(current_result, nums[index], operator);
    if index == nums.len() {
        results.push(next_result);
    } else {
        calculate_from(nums, results, next_result, index + 1, Operator::ADD);
        calculate_from(nums, results, next_result, index + 1, Operator::MULTIPLY);
    }
}

fn perform_operation(a: u32, b: u32, operator: Operator) -> u32 {
    match operator {
        Operator::ADD => a + b,
        Operator::MULTIPLY => a * b,
    }
}

pub fn solve2(path: &str) {}
