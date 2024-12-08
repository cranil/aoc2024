use std::collections::{HashMap, HashSet};

use algos::grid::{Grid, RectangularGrid};

#[allow(unused_variables)]
fn solve1(input: &str) -> usize {
    let lines = input.lines();
    let height = lines.clone().count();
    let width = lines.clone().next().unwrap().len();
    let mut grid = RectangularGrid::new(width, height);
    let mut antennas = HashMap::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    antennas.entry(c).or_insert_with(Vec::new).push((x, y));
                }
                _ => {}
            }
            grid.set(x, y, c);
        }
    }

    let mut antinodes = HashSet::new();
    for (antenna, positions) in antennas {
        let width = width as isize;
        let height = height as isize;

        for pos0 in positions.iter() {
            for pos1 in positions.iter().filter(|&pos| pos != pos0) {
                let pos0 = (pos0.0 as isize, pos0.1 as isize);
                let pos1 = (pos1.0 as isize, pos1.1 as isize);

                let dx = pos1.0 - pos0.0;
                let dy = pos1.1 - pos0.1;

                let x0 = pos0.0 + 2 * dx;
                let y0 = pos0.1 + 2 * dy;

                let x1 = pos1.0 - 2 * dx;
                let y1 = pos1.1 - 2 * dy;

                if x0 >= 0 && x0 < width && y0 >= 0 && y0 < height {
                    antinodes.insert((x0, y0));
                }

                if x1 >= 0 && x1 < width && y1 >= 0 && y1 < height {
                    antinodes.insert((x1, y1));
                }
            }
        }
    }
    antinodes.len()
}

#[allow(unused_variables)]
fn solve2(input: &str) -> usize {
    let lines = input.lines();
    let height = lines.clone().count();
    let width = lines.clone().next().unwrap().len();
    let mut grid = RectangularGrid::new(width, height);
    let mut antennas = HashMap::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    antennas.entry(c).or_insert_with(Vec::new).push((x, y));
                }
                _ => {}
            }
            grid.set(x, y, c);
        }
    }

    let mut antinodes = HashSet::new();
    for (antenna, positions) in antennas {
        let width = width as isize;
        let height = height as isize;

        for pos0 in positions.iter() {
            for pos1 in positions.iter().filter(|&pos| pos != pos0) {
                let pos0 = (pos0.0 as isize, pos0.1 as isize);
                let pos1 = (pos1.0 as isize, pos1.1 as isize);

                let dx = pos1.0 - pos0.0;
                let dy = pos1.1 - pos0.1;

                let mut x = pos0.0;
                let mut y = pos0.1;

                while x >= 0 && x < width && y >= 0 && y < height {
                    antinodes.insert((x, y));
                    x += dx;
                    y += dy;
                }

                let mut x = pos0.0;
                let mut y = pos0.1;

                while x >= 0 && x < width && y >= 0 && y < height {
                    antinodes.insert((x, y));
                    x -= dx;
                    y -= dy;
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
