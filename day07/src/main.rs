fn operators_from_usize(mut value: usize, len: usize) -> Vec<char> {
    let mut operators = Vec::with_capacity(len - 1);
    for _ in 0..len - 1 {
        let operator = match value & 1 {
            0 => '+',
            _ => '*',
        };
        value >>= 1;
        operators.push(operator);
    }
    operators
}

fn solve1(input: &str) -> usize {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let mut parts = line.split(":");
        let value = parts.next().unwrap().parse::<usize>().unwrap();
        let operands = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        for i in 0..(1 << (operands.len() - 1)) {
            let operators = operators_from_usize(i, operands.len());
            let mut result = operands[0];
            for (operator, operand) in
                operators.iter().zip(operands.iter().skip(1))
            {
                match operator {
                    '+' => result += operand,
                    '*' => result *= operand,
                    _ => panic!("Unknown operator"),
                }
            }
            if result == value {
                total += value;
                break;
            }
        }
    }
    total
}

fn operators_from_usize2(mut value: usize, len: usize) -> Vec<char> {
    let mut operators = Vec::with_capacity(len - 1);
    for _ in 0..len - 1 {
        let operator = match value % 3 {
            0 => '+',
            1 => '*',
            2 => 'c',
            _ => unreachable!(),
        };
        value /= 3;
        operators.push(operator);
    }
    operators
}

fn solve2(input: &str) -> usize {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let mut parts = line.split(":");
        let value = parts.next().unwrap().parse::<usize>().unwrap();
        let operands = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        for i in 0..(3_usize.pow((operands.len() - 1) as u32)) {
            let operators = operators_from_usize2(i, operands.len());
            let mut result = operands[0];
            for (operator, operand) in
                operators.iter().zip(operands.iter().skip(1))
            {
                match operator {
                    '+' => result += operand,
                    '*' => result *= operand,
                    'c' => {
                        result = format!("{}{}", result, operand)
                            .parse::<usize>()
                            .unwrap()
                    }
                    _ => panic!("Unknown operator"),
                }
            }
            if result == value {
                total += value;
                break;
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input)
}
