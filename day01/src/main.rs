use std::collections::HashMap;

fn solve1(input: &str) -> u64 {
    let lines = input.lines();
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    lines.for_each(|line| {
        let s = line.splitn(2, "   ").collect::<Vec<_>>();
        col1.push(s[0].parse::<i64>().unwrap());
        col2.push(s[1].parse::<i64>().unwrap());
    });
    col1.sort();
    col2.sort();
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += (col1[i] - col2[i]).abs();
    }
    sum as u64
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();
    let mut col1 = Vec::new();
    let mut col2 = HashMap::new();
    lines.for_each(|line| {
        let s = line.splitn(2, "   ").collect::<Vec<_>>();
        col1.push(s[0].parse::<usize>().unwrap());
        match col2.entry(s[1].parse::<usize>().unwrap()) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1_usize);
            }
        }
    });
    col1.sort();
    col1.iter()
        .map(|x| *x * *col2.get(x).unwrap_or(&0))
        .sum::<usize>() as u64
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
