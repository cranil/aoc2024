use std::collections::HashSet;

use algos::grid::{Grid, RectangularGrid};

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (isize, isize)> {
    static DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    DIRS.iter()
        .map(move |&(dx, dy)| (x as isize + dx, y as isize + dy))
}

fn solve1(grid: &RectangularGrid<char>) -> usize {
    let mut total_cost = 0;
    let mut visited = RectangularGrid::new(grid.width, grid.height);
    visited.fill(false);

    for ch in 'A'..='Z' {
        let mut region = HashSet::new();

        for y in 0..grid.height {
            for x in 0..grid.width {
                if *grid.at_unchecked(x, y) == ch {
                    region.insert((x, y));
                }
            }
        }

        while !region.is_empty() {
            let mut perimeter = 0;

            let mut stack = Vec::new();
            let start = region.iter().next().unwrap();
            stack.push(*start);
            while let Some((x, y)) = stack.pop() {
                if *visited.at_unchecked(x, y) {
                    continue;
                }
                visited.set(x, y, true);
                for (nx, ny) in neighbors(x, y) {
                    if nx < 0 || ny < 0 {
                        continue;
                    }
                    if nx >= grid.width as isize || ny >= grid.height as isize {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if region.contains(&(nx, ny)) {
                        stack.push((nx, ny));
                    }
                }
            }

            let subregion = visited
                .iter()
                .filter(|(_, _, v)| **v)
                .map(|(x, y, _)| (x, y))
                .collect::<Vec<_>>();

            let area = subregion.len();

            for (x, y) in subregion.iter() {
                for (nx, ny) in neighbors(*x, *y) {
                    if nx < 0 || ny < 0 {
                        perimeter += 1;
                        continue;
                    }
                    if nx >= grid.width as isize || ny >= grid.height as isize {
                        perimeter += 1;
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !*visited.at_unchecked(nx, ny) && !region.contains(&(nx, ny)) {
                        perimeter += 1;
                    }
                }
            }

            let cost = area * perimeter;
            for value in subregion.iter() {
                region.remove(value);
            }

            visited.fill(false);
            total_cost += cost;
        }
    }
    total_cost
}

fn solve2(grid: &RectangularGrid<char>) -> usize {
    let mut total_cost = 0;
    let mut visited = RectangularGrid::new(grid.width, grid.height);
    visited.fill(false);

    for ch in 'A'..='Z' {
        let mut region = HashSet::new();

        for y in 0..grid.height {
            for x in 0..grid.width {
                if *grid.at_unchecked(x, y) == ch {
                    region.insert((x, y));
                }
            }
        }

        while !region.is_empty() {
            let mut stack = Vec::new();
            let start = region.iter().next().unwrap();
            stack.push(*start);
            while let Some((x, y)) = stack.pop() {
                if *visited.at_unchecked(x, y) {
                    continue;
                }
                visited.set(x, y, true);
                for (nx, ny) in neighbors(x, y) {
                    if nx < 0 || ny < 0 {
                        continue;
                    }
                    if nx >= grid.width as isize || ny >= grid.height as isize {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if region.contains(&(nx, ny)) {
                        stack.push((nx, ny));
                    }
                }
            }

            let subregion = visited
                .iter()
                .filter(|(_, _, v)| **v)
                .map(|(x, y, _)| (x, y))
                .collect::<Vec<_>>();

            let area = subregion.len();

            let mut sidesx = 0;
            let minx = *subregion.iter().map(|(x, _)| x).min().unwrap();
            let maxx = *subregion.iter().map(|(x, _)| x).max().unwrap();
            for x in minx..=maxx {
                let mut sides_left = false;
                let mut sides_right = false;
                for y in 0..grid.height {
                    if *visited.at_unchecked(x, y) {
                        if x == 0 && !sides_left {
                            sides_left = true;
                            sidesx += 1;
                        }
                        if x == grid.width - 1 && !sides_right {
                            sides_right = true;
                            sidesx += 1;
                        }
                        if x > 0 {
                            if !*visited.at_unchecked(x - 1, y) {
                                if !sides_left {
                                    sides_left = true;
                                    sidesx += 1;
                                }
                            } else if sides_left {
                                sides_left = false;
                            }
                        }
                        if x < grid.width - 1 {
                            if !*visited.at_unchecked(x + 1, y) {
                                if !sides_right {
                                    sides_right = true;
                                    sidesx += 1;
                                }
                            } else if sides_right {
                                sides_right = false;
                            }
                        }
                    } else {
                        sides_left = false;
                        sides_right = false;
                    }
                }
            }

            let miny = *subregion.iter().map(|(_, y)| y).min().unwrap();
            let maxy = *subregion.iter().map(|(_, y)| y).max().unwrap();

            let mut sidesy = 0;
            for y in miny..=maxy {
                let mut sides_top = false;
                let mut sides_bottom = false;
                for x in 0..grid.width {
                    if *visited.at_unchecked(x, y) {
                        if y == 0 && !sides_top {
                            sides_top = true;
                            sidesy += 1;
                        }
                        if y == grid.height - 1 && !sides_bottom {
                            sides_bottom = true;
                            sidesy += 1;
                        }
                        if y > 0 {
                            if !*visited.at_unchecked(x, y - 1) {
                                if !sides_top {
                                    sides_top = true;
                                    sidesy += 1;
                                }
                            } else if sides_top {
                                sides_top = false;
                            }
                        }
                        if y < grid.height - 1 {
                            if !*visited.at_unchecked(x, y + 1) {
                                if !sides_bottom {
                                    sides_bottom = true;
                                    sidesy += 1;
                                }
                            } else if sides_bottom {
                                sides_bottom = false;
                            }
                        }
                    } else {
                        sides_top = false;
                        sides_bottom = false;
                    }
                }
            }

            let sides = sidesx + sidesy;
            let cost = area * sides;
            for value in subregion.iter() {
                region.remove(value);
            }

            visited.fill(false);
            total_cost += cost;
        }
    }
    total_cost
}

fn main() {
    let input = include_str!("../input.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut grid = RectangularGrid::new(width, height);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c);
        }
    }

    let fs = [solve1, solve2];
    solver::solver(fs, &grid);
}
