use regex::Regex;

fn do_mul(s: &str) -> i64 {
    let mut idx = 4;

    let mut first = 0;
    while s.chars().nth(idx).unwrap().is_ascii_digit() {
        first = first * 10 + s.chars().nth(idx).unwrap().to_digit(10).unwrap() as i64;
        idx += 1;
    }

    idx += 1; //,

    let mut second = 0;
    while s.chars().nth(idx).unwrap().is_ascii_digit() {
        second = second * 10 + s.chars().nth(idx).unwrap().to_digit(10).unwrap() as i64;
        idx += 1;
    }

    first * second
}

fn find_muls(input: &str) -> i64 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut sum = 0;
    for m in re.find_iter(input) {
        let s = m.as_str();
        sum += do_mul(s);
    }
    sum
}

fn find_cond_muls(input: &str) -> i64 {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let mut sum = 0;
    let mut flag = true;
    for m in re.find_iter(input) {
        let s = m.as_str();
        if s.starts_with("mul") && flag {
            sum += do_mul(s);
        } else if s.starts_with("don't") {
            flag = false;
        } else if s.starts_with("do") {
            flag = true;
        }
    }
    sum
}

fn solve1(input: &str) -> i64 {
    find_muls(input)
}

fn solve2(input: &str) -> i64 {
    find_cond_muls(input)
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
