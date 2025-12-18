pub fn solve1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines: Vec<(u64, Vec<u64>)> = input.lines().map(parse_line).collect();
    let mut sum = 0u64;
    for (result, nums) in lines {
        let mut results: Vec<u64> = Vec::new();
        if nums.len() > 0 {
            calculate_from(&nums, &mut results, nums[0], 1, Operator::ADD);
            calculate_from(&nums, &mut results, nums[0], 1, Operator::MULTIPLY);
            calculate_from(&nums, &mut results, nums[0], 1, Operator::CONCAT);
        }
        if results.contains(&result) {
            sum += result;
        }
    }
    dbg!(sum);
}

enum Operator {
    MULTIPLY,
    ADD,
    CONCAT,
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        panic!("Invalid line format: {}", line);
    }
    let result: u64 = parts[0]
        .trim()
        .parse()
        .expect(&format!("Failed to parse result: '{}'", parts[0]));
    let nums: Vec<u64> = parts[1]
        .trim()
        .split_whitespace()
        .map(|s| {
            s.parse()
                .expect(&format!("Failed to parse number: '{}'", s))
        })
        .collect();
    return (result, nums);
}

fn calculate_from(
    nums: &Vec<u64>,
    results: &mut Vec<u64>,
    current_result: u64,
    index: usize,
    operator: Operator,
) {
    let next_result = perform_operation(current_result, nums[index], operator);
    if index == nums.len() - 1 {
        results.push(next_result);
    } else {
        calculate_from(nums, results, next_result, index + 1, Operator::ADD);
        calculate_from(nums, results, next_result, index + 1, Operator::MULTIPLY);
        calculate_from(nums, results, next_result, index + 1, Operator::CONCAT);
    }
}

fn perform_operation(a: u64, b: u64, operator: Operator) -> u64 {
    match operator {
        Operator::ADD => a + b,
        Operator::MULTIPLY => a * b,
        Operator::CONCAT => {
            let mut new_number = a.to_string();
            new_number.push_str(&b.to_string());
            new_number.parse().unwrap()
        }
    }
}

pub fn solve2(path: &str) {}
