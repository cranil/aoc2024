use std::collections::HashSet;

use algos::fnv_hash_map::Fnv1aHashSet;

use algos::grid::{Grid, RectangularGrid};

fn do_rounds(
    grid: Option<RectangularGrid<char>>,
    &(obstacles, start): &(&Fnv1aHashSet<(usize, usize)>, (usize, usize)),
) -> Option<Fnv1aHashSet<(usize, usize)>> {
    let mut current = start;
    let mut dir = (0, -1);

    let (height, width) = if let Some(ref grid) = grid {
        (grid.height, grid.width)
    } else {
        let width = obstacles.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = obstacles.iter().map(|(_, y)| y).max().unwrap() + 1;
        (height, width)
    };

    let mut visited = Fnv1aHashSet::default();
    let mut path = Fnv1aHashSet::default();
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
                    if grid.is_some() {
                        for y in 0..current.1 {
                            path.insert((current.0, y));
                        }
                    }
                }
                (1, 0) => {
                    if grid.is_some() {
                        for x in current.0 + 1..width {
                            path.insert((x, current.1));
                        }
                    }
                }
                (0, 1) => {
                    if grid.is_some() {
                        for y in current.1 + 1..height {
                            path.insert((current.0, y));
                        }
                    }
                }
                (-1, 0) => {
                    if grid.is_some() {
                        for x in 0..current.0 {
                            path.insert((x, current.1));
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
                    if grid.is_some() {
                        for y in next.1 + 1..=current.1 {
                            path.insert((next.0, y));
                        }
                    }

                    current.1 = next.1 + 1;
                }
                (1, 0) => {
                    if grid.is_some() {
                        for x in current.0 + 1..next.0 {
                            path.insert((x, next.1));
                        }
                    }
                    current.0 = next.0 - 1;
                }
                (0, 1) => {
                    if grid.is_some() {
                        for y in current.1..next.1 {
                            path.insert((next.0, y));
                        }
                    }
                    current.1 = next.1 - 1;
                }
                (-1, 0) => {
                    if grid.is_some() {
                        for x in next.0 + 1..=current.0 {
                            path.insert((x, next.1));
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

    Some(path)
}

fn solve1(input: &(&Fnv1aHashSet<(usize, usize)>, (usize, usize))) -> usize {
    let width = input.0.iter().map(|&(x, _)| x).max().unwrap() + 1;
    let height = input.0.iter().map(|&(_, y)| y).max().unwrap() + 1;
    let mut grid = RectangularGrid::new(width, height);
    grid.fill('.');
    for &(x, y) in input.0.iter() {
        grid.set(x, y, '#');
    }

    do_rounds(Some(grid), input).unwrap().len()
}

fn solve2(input: &(&Fnv1aHashSet<(usize, usize)>, (usize, usize))) -> usize {
    let width = *input.0.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = *input.0.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut obstacles = input.0.clone();

    let mut grid = RectangularGrid::new(width, height);

    grid.fill('.');
    for &(x, y) in input.0.iter() {
        grid.set(x, y, '#');
    }
    let path = do_rounds(Some(grid), input).unwrap();

    let mut candidates = Fnv1aHashSet::default();

    let mut count = 0;
    for (x, y) in path {
        candidates.insert((x, y));
        if x > 0 {
            if obstacles.contains(&(x - 1, y)) {
                continue;
            }
            candidates.insert((x - 1, y));
        }
        if x < width - 1 {
            if obstacles.contains(&(x + 1, y)) {
                continue;
            }
            candidates.insert((x + 1, y));
        }
        if y > 0 {
            if obstacles.contains(&(x, y - 1)) {
                continue;
            }
            candidates.insert((x, y - 1));
        }
        if y < height - 1 {
            if obstacles.contains(&(x, y + 1)) {
                continue;
            }
            candidates.insert((x, y + 1));
        }
    }

    for (x, y) in candidates {
        obstacles.insert((x, y));
        if do_rounds(None, &(&obstacles, input.1)).is_none() {
            count += 1;
        }
        obstacles.remove(&(x, y));
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    let mut obstacles = Fnv1aHashSet::default();
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
