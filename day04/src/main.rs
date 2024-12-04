fn solve1(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid = vec![vec![' '; lines[0].len()]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }
    let mut count = 0;
    const XMAS: &str = "XMAS";
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                // horizontal
                let mut found = true;
                for k in 0..4 {
                    if j + k >= grid[0].len() {
                        found = false;
                        break;
                    }
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

                found = true;
                // horizontal reverse
                for k in 0..4 {
                    if (j as i32) - (k as i32) < 0 {
                        found = false;
                        break;
                    }
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

                found = true;

                // vertical
                for k in 0..4 {
                    if i + k >= grid.len() {
                        found = false;
                        break;
                    }
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

                found = true;

                // vertical reverse

                for k in 0..4 {
                    if (i as i32) - (k as i32) < 0 {
                        found = false;
                        break;
                    }
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

                found = true;

                // diagonal
                for k in 0..4 {
                    if i + k >= grid.len() || j + k >= grid[0].len() {
                        found = false;
                        break;
                    }
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

                found = true;

                // diagonal reverse

                for k in 0..4 {
                    if (i as i32) - (k as i32) < 0
                        || (j as i32) - (k as i32) < 0
                    {
                        found = false;
                        break;
                    }
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

                found = true;

                // off-diagonal

                for k in 0..4 {
                    if (i as i32) - (k as i32) < 0 || j + k >= grid[0].len() {
                        found = false;
                        break;
                    }
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

                found = true;

                // off-diagonal reverse

                for k in 0..4 {
                    if i + k >= grid.len() || (j as i32) - (k as i32) < 0 {
                        found = false;
                        break;
                    }
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
    count
}

fn solve2(input: &str) -> i64 {
    let mut count = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid = vec![vec![' '; lines[0].len()]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

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
                }
            }
        }
    }
    count
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
