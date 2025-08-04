use std::fs;
pub fn solve(path: String) {
    let contents = fs::read_to_string(path).expect("Couldnt read from file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let reports_vec: Vec<Vec<i32>> = lines
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse().expect("Parse error"))
                .collect()
        })
        .collect();
    let mut count = 0;
    for report in reports_vec {
        print!("{report:?} = ");
        if is_valid(report, true) {
            count += 1;
            println!("VALID");
        } else {
            println!("NOT valid");
        }
    }

    println!("{count}");
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three
fn is_valid(report: Vec<i32>, extra_life: bool) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let delta_range;
    if report.get(0) > report.get(1) {
        delta_range = -3..=-1;
    } else if report.get(0) < report.get(1) {
        delta_range = 1..=3;
    } else if extra_life {
        let shortened_1 = remove_index(&report, 0);
        let shortened_2 = remove_index(&report, 1);
        return is_valid(shortened_1, false) || is_valid(shortened_2, false);
    } else {
        return false;
    }

    for i in 0..(report.len() - 1) {
        let delta = report.get(i + 1).unwrap() - report.get(i).unwrap();
        if !delta_range.contains(&delta) {
            if !extra_life {
                return false;
            } else {
                let shortened_1 = remove_index(&report, i);
                let shortened_2 = remove_index(&report, i + 1);
                let result = is_valid(shortened_1, false) || is_valid(shortened_2, false);
                if i == 0 {
                    return result;
                }
                let shortened_3 = remove_index(&report, i - 1);

                return result || is_valid(shortened_3, false);
            }
        }
    }
    true
}

fn remove_index(report: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut shortened_report: Vec<i32> = Vec::new();
    for j in 0..report.len() {
        if j != index {
            shortened_report.push(*report.get(j).unwrap());
        }
    }
    shortened_report
}

pub fn debug() {
    // 7 6 4 2 1: Safe without removing any level.
    // 1 2 7 8 9: Unsafe regardless of which level is removed.
    // 9 7 6 2 1: Unsafe regardless of which level is removed.
    // 1 3 2 4 5: Safe by removing the second level, 3.
    // 8 6 4 4 1: Safe by removing the third level, 4.
    // 1 3 6 7 9: Safe without removing any level.
    let report1: Vec<i32> = vec![7, 6, 4, 2, 1];
    let report2: Vec<i32> = vec![1, 2, 7, 8, 9];
    let report3: Vec<i32> = vec![9, 7, 6, 2, 1];
    let report4: Vec<i32> = vec![1, 3, 2, 4, 5];
    let report5: Vec<i32> = vec![8, 6, 4, 4, 1];
    let report6: Vec<i32> = vec![1, 3, 6, 7, 9];

    println!(
        "Chat: [4, 1, 4, 7]: is_valid = {}",
        is_valid(vec![4, 1, 4, 7], true)
    );
}
