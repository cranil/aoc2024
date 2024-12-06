use std::collections::HashSet;

use algos::grid::{Grid, RectangularGrid};

fn do_rounds(
    mut grid: Option<RectangularGrid<char>>,
    &(obstacles, start): &(&HashSet<(usize, usize)>, (usize, usize)),
) -> Option<usize> {
    let mut current = start;
    let mut dir = (0, -1);

    let (height, width) = if let Some(ref grid) = grid {
        (grid.height, grid.width)
    } else {
        let width = obstacles.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = obstacles.iter().map(|(_, y)| y).max().unwrap() + 1;
        (height, width)
    };

    let mut visited = HashSet::new();
    loop {
        let mut os = match dir {
            (0, -1) => obstacles
                .iter()
                .filter(|(x, y)| current.0 == *x && *y < current.1)
                .collect::<Vec<_>>(),
            (1, 0) => obstacles
                .iter()
                .filter(|(x, y)| current.1 == *y && *x > current.0)
                .collect::<Vec<_>>(),
            (0, 1) => obstacles
                .iter()
                .filter(|(x, y)| current.0 == *x && *y > current.1)
                .collect::<Vec<_>>(),
            (-1, 0) => obstacles
                .iter()
                .filter(|(x, y)| current.1 == *y && *x < current.0)
                .collect::<Vec<_>>(),
            _ => unreachable!(),
        };
        os.sort_by(|(x1, y1), (x2, y2)| match dir {
            (0, -1) => y1.cmp(y2),
            (1, 0) => x2.cmp(x1),
            (0, 1) => y2.cmp(y1),
            (-1, 0) => x1.cmp(x2),
            _ => unreachable!(),
        });
        if os.is_empty() {
            match dir {
                (0, -1) => {
                    if let Some(ref mut grid) = grid {
                        for y in 0..current.1 {
                            grid.set(current.0, y, 'X');
                        }
                    }
                }
                (1, 0) => {
                    if let Some(ref mut grid) = grid {
                        for x in current.0 + 1..width {
                            grid.set(x, current.1, 'X');
                        }
                    }
                }
                (0, 1) => {
                    if let Some(ref mut grid) = grid {
                        for y in current.1 + 1..height {
                            grid.set(current.0, y, 'X');
                        }
                    }
                }
                (-1, 0) => {
                    if let Some(ref mut grid) = grid {
                        for x in 0..current.0 {
                            grid.set(x, current.1, 'X');
                        }
                    }
                }
                _ => unreachable!(),
            }
            break;
        } else {
            let next = *os.last().unwrap();
            if visited.contains(&(next.0, next.1, dir)) {
                return None;
            }
            visited.insert((next.0, next.1, dir));
            match dir {
                (0, -1) => {
                    if let Some(ref mut grid) = grid {
                        for y in next.1 + 1..=current.1 {
                            grid.set(next.0, y, 'X');
                        }
                    }
                    current.1 = next.1 + 1;
                }
                (1, 0) => {
                    if let Some(ref mut grid) = grid {
                        for x in current.0 + 1..=next.0 {
                            grid.set(x, next.1, 'X');
                        }
                    }
                    current.0 = next.0 - 1;
                }
                (0, 1) => {
                    if let Some(ref mut grid) = grid {
                        for y in current.1..next.1 {
                            grid.set(next.0, y, 'X');
                        }
                    }
                    current.1 = next.1 - 1;
                }
                (-1, 0) => {
                    if let Some(ref mut grid) = grid {
                        for x in next.0 + 1..=current.0 {
                            grid.set(x, next.1, 'X');
                        }
                    }
                    current.0 = next.0 + 1;
                }
                _ => unreachable!(),
            };
        }
        dir = match dir {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => unreachable!(),
        };
    }
    let mut count = 0;
    if let Some(ref grid) = grid {
        for y in 0..height {
            for x in 0..width {
                if grid.at(x, y).unwrap() == &'X' {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

fn solve1(input: &(&HashSet<(usize, usize)>, (usize, usize))) -> usize {
    let width = *input.0.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = *input.0.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut grid = RectangularGrid::new(width, height);
    grid.fill('.');
    for (x, y) in input.0.iter() {
        grid.set(*x, *y, '#');
    }

    do_rounds(Some(grid), input).unwrap()
}

fn solve2(input: &(&HashSet<(usize, usize)>, (usize, usize))) -> usize {
    let width = *input.0.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = *input.0.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut obstacles = input.0.clone();

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if obstacles.contains(&(x, y)) {
                continue;
            } else {
                obstacles.insert((x, y));
            }
            if do_rounds(None, &(&obstacles, input.1)).is_none() {
                count += 1;
            }
            obstacles.remove(&(x, y));
        }
    }
    count
}

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    // let mut input = RectangularGrid::new(lines[0].len(), lines.len());
    let mut obstacles = HashSet::new();
    let mut start = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert((x, y));
            } else if c == '^' {
                start = (x, y);
            }
        }
    }
    let fs = [solve1, solve2];
    solver::solver(fs, &(&obstacles, start));
}
