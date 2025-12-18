use regex::Regex;
use std::fs;

pub fn solve1(path: String) {
    let re = Regex::new("mul\\([0-9]+,[0-9]+\\)").unwrap();
    let contents = fs::read_to_string(path).expect("File read error");
    let result: i32 = calculate(&contents.as_str(), &re);
    println!("{result}");
}

fn calculate(s: &str, mul_regex: &Regex) -> i32 {
    let result: i32 = mul_regex
        .find_iter(&s)
        .map(|arg0: regex::Match<'_>| parse_match(arg0.as_str()))
        .sum();
    result
}

pub fn solve2(path: String) {
    let mul_re = Regex::new("mul\\([0-9]+,[0-9]+\\)").unwrap();
    let do_re = Regex::new("do\\(\\)").unwrap();
    let dont_re = Regex::new("don't\\(\\)").unwrap();
    let do_s = "do()";
    let dont_s = "don't()";

    let contents = fs::read_to_string(path).expect("File read error");

    let start_is_valid: Vec<&str> = contents.split(do_s).collect();
    let mut valid_strings: Vec<&str> = Vec::new();
    for s in start_is_valid {
        let first_dont = s.find(dont_s);
        if first_dont.is_none() {
            valid_strings.push(s);
        } else {
            valid_strings.push(&s[0..first_dont.expect("Unexpected")]);
        }
    }

    let mut result = 0;

    for s in valid_strings {
        result += calculate(&s, &mul_re);
    }
    println!("{result}");
}

fn parse_match(mul: &str) -> i32 {
    return mul
        .trim_start_matches("mul(")
        .trim_end_matches(")")
        .split(",")
        .map(|s| s.parse().expect("Parse error"))
        .reduce(|acc: i32, e: i32| acc * e)
        .unwrap();
}
