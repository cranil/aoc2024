use std::collections::HashMap;

fn stone_splitter(num: usize, iters: usize, memoize: &mut HashMap<(usize, usize), usize>) -> usize {
    if iters == 0 {
        return 1;
    }
    if let Some(&val) = memoize.get(&(num, iters)) {
        return val;
    }
    let digits = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    if num == 0 {
        let num_stones = stone_splitter(1, iters - 1, memoize);
        memoize.insert((num, iters), num_stones);
        num_stones
    } else if digits.len() % 2 == 0 {
        let num1 = digits
            .iter()
            .take(digits.len() / 2)
            .fold(0, |acc, x| acc * 10 + x);
        let num2 = digits
            .iter()
            .skip(digits.len() / 2)
            .fold(0, |acc, x| acc * 10 + x);
        let num_stones =
            stone_splitter(num1, iters - 1, memoize) + stone_splitter(num2, iters - 1, memoize);
        memoize.insert((num, iters), num_stones);
        num_stones
    } else {
        let num1 = num * 2024;
        let num_stones = stone_splitter(num1, iters - 1, memoize);
        memoize.insert((num, iters), num_stones);
        num_stones
    }
}

fn solve1(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut total = 0;
    for stone in stones {
        let num_stones = stone_splitter(stone, 25, &mut HashMap::new());
        total += num_stones;
    }
    total
}

fn solve2(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut total = 0;
    for stone in stones {
        let num_stones = stone_splitter(stone, 75, &mut HashMap::new());
        total += num_stones;
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
