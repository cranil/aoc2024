use algos::fnv_hash_map::Fnv1aHashMap;

fn stone_splitter(
    num: usize,
    iters: usize,
    memoize: &mut [Fnv1aHashMap<usize, usize>],
) -> usize {
    if iters == 0 {
        return 1;
    }
    if let Some(&val) = memoize[iters - 1].get(&num) {
        return val;
    }

    let n_digits = (num as f32).log10().floor() as u32 + 1;
    let split = 10_usize.pow(n_digits / 2);
    if num == 0 {
        let num_stones = stone_splitter(1, iters - 1, memoize);
        memoize[iters - 1].insert(num, num_stones);
        num_stones
    } else if n_digits % 2 == 0 {
        let num1 = num / split;
        let num2 = num % split;
        let num_stones = stone_splitter(num1, iters - 1, memoize)
            + stone_splitter(num2, iters - 1, memoize);
        memoize[iters - 1].insert(num, num_stones);
        num_stones
    } else {
        let num1 = num * 2024;
        let num_stones = stone_splitter(num1, iters - 1, memoize);
        memoize[iters - 1].insert(iters, num_stones);
        num_stones
    }
}

fn solve1(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut total = 0;
    let mut memoize = vec![Fnv1aHashMap::default(); 25];
    for stone in stones {
        let num_stones = stone_splitter(stone, 25, &mut memoize);
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
    let mut memoize = vec![Fnv1aHashMap::default(); 75];
    for stone in stones {
        let num_stones = stone_splitter(stone, 75, &mut memoize);
        total += num_stones;
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
