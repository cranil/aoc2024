fn is_completely_safe(levels: &[i64]) -> bool {
    let mut prev_sign = None;
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        let sign = if diff > 0 {
            1
        } else if diff < 0 {
            -1
        } else {
            0
        };
        if let Some(prev_sign) = prev_sign {
            if sign != prev_sign {
                return false;
            }
        } else {
            prev_sign = Some(sign);
        }
    }
    true
}

fn is_safe(levels: &[i64]) -> bool {
    if is_completely_safe(levels) {
        return true;
    }
    for level in 0..levels.len() {
        let levels2 = levels
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != level)
            .map(|(_, &l)| l)
            .collect::<Vec<_>>();
        if is_completely_safe(&levels2) {
            return true;
        }
    }
    false
}

fn solve1(input: &str) -> i64 {
    let lines = input.lines();
    let records = lines
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let count = records
        .iter()
        .map(|levels| if is_completely_safe(levels) { 1 } else { 0 })
        .sum::<i64>();
    count
}

fn solve2(input: &str) -> i64 {
    let lines = input.lines();
    let records = lines
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    records
        .iter()
        .map(|levels| if is_safe(levels) { 1 } else { 0 })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
