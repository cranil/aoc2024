fn solve1(grid: &[Vec<char>]) -> i64 {
    let mut count = 0;
    const XMAS: &str = "XMAS";
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                // horizontal
                if j + 3 < grid[0].len() {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i][j + k] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }

                // horizontal reverse
                if j >= 3 {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i][j - k] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }

                // vertical
                if i + 3 < grid.len() {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i + k][j] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        count += 1;
                    }
                }

                // vertical reverse
                if i >= 3 {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i - k][j] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        count += 1;
                    }
                }

                // diagonal
                if i + 3 < grid.len() && j + 3 < grid[0].len() {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i + k][j + k] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        count += 1;
                    }
                }

                // diagonal reverse
                if i >= 3 && j >= 3 {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i - k][j - k] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        count += 1;
                    }
                }

                // off-diagonal
                if i >= 3 && j + 3 < grid[0].len() {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i - k][j + k] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        count += 1;
                    }
                }

                // off-diagonal reverse
                if i + 3 < grid.len() && j >= 3 {
                    let mut found = true;
                    for k in 0..4 {
                        if grid[i + k][j - k] == XMAS.chars().nth(k).unwrap() {
                            continue;
                        } else {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn solve2(grid: &[Vec<char>]) -> i64 {
    let mut count = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] == 'A' {
                // M.M
                // .A.
                // S.S
                let mut cond = grid[i - 1][j - 1] == 'M';
                cond = cond && grid[i - 1][j + 1] == 'M';
                cond = cond && grid[i + 1][j + 1] == 'S';
                cond = cond && grid[i + 1][j - 1] == 'S';
                if cond {
                    count += 1;
                    continue;
                }

                // S.S
                // .A.
                // M.M
                cond = grid[i - 1][j - 1] == 'S';
                cond = cond && grid[i - 1][j + 1] == 'S';
                cond = cond && grid[i + 1][j + 1] == 'M';
                cond = cond && grid[i + 1][j - 1] == 'M';
                if cond {
                    count += 1;
                    continue;
                }

                // M.S
                // .A.
                // M.S
                cond = grid[i - 1][j - 1] == 'M';
                cond = cond && grid[i - 1][j + 1] == 'S';
                cond = cond && grid[i + 1][j + 1] == 'S';
                cond = cond && grid[i + 1][j - 1] == 'M';
                if cond {
                    count += 1;
                    continue;
                }

                // S.M
                // .A.
                // S.M
                cond = grid[i - 1][j - 1] == 'S';
                cond = cond && grid[i - 1][j + 1] == 'M';
                cond = cond && grid[i + 1][j + 1] == 'M';
                cond = cond && grid[i + 1][j - 1] == 'S';
                if cond {
                    count += 1;
                    continue;
                }
            }
        }
    }
    count
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let fs = [solve1, solve2];
    solver::solver(fs, &grid);
}
