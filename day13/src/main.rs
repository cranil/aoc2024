fn solve(input: &str, c: isize) -> isize {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    for chunk in lines.chunks(4) {
        let mut iter = chunk.iter();
        let cons1 = iter.next().unwrap();
        let cons2 = iter.next().unwrap();
        let prize = iter.next().unwrap();

        let mut split = cons1.split(": ");
        let mut split = split.nth(1).unwrap().split(", ");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        let a0 = a.split("+").nth(1).unwrap().parse::<isize>().unwrap();
        let a1 = b.split("+").nth(1).unwrap().parse::<isize>().unwrap();

        let mut split = cons2.split(": ");
        let mut split = split.nth(1).unwrap().split(", ");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        let b0 = a.split("+").nth(1).unwrap().parse::<isize>().unwrap();
        let b1 = b.split("+").nth(1).unwrap().parse::<isize>().unwrap();

        let mut split = prize.split(": ");
        let mut split = split.nth(1).unwrap().split(", ");
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let x = x.split("=").nth(1).unwrap().parse::<isize>().unwrap() + c;
        let y = y.split("=").nth(1).unwrap().parse::<isize>().unwrap() + c;

        let denom = a0 * b1 - a1 * b0;
        let num1 = a0 * y - a1 * x;
        let num2 = b1 * x - b0 * y;

        if denom == 0 {
            continue;
        }
        if num1 % denom != 0 || num2 % denom != 0 {
            continue;
        }

        let i = num1 / denom;
        let j = num2 / denom;

        if i < 0 || j < 0 {
            continue;
        }

        sum += i + 3 * j;

        let _ = iter.next();
    }
    sum
}

fn solve1(input: &str) -> isize {
    solve(input, 0)
}

fn solve2(input: &str) -> isize {
    solve(input, 10000000000000)
}

fn main() {
    let input = include_str!("../input.txt");

    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
