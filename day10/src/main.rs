use algos::{
    fnv_hash_map::{Fnv1aHashMap, Fnv1aHashSet},
    grid::{Grid, RectangularGrid},
};

fn parse_grid(input: &str) -> RectangularGrid<u8> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut grid = RectangularGrid::new(width, height);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let c = c as u8 - b'0';
            grid.set(x, y, c);
        }
    }
    grid
}

fn get_neighbors(
    grid: &RectangularGrid<u8>,
    x: usize,
    y: usize,
    height: u8,
) -> Vec<(usize, usize)> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .filter_map(|(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;
            if x < 0 || y < 0 || x >= grid.width as isize || y >= grid.height as isize {
                None
            } else if *grid.at(x as usize, y as usize).unwrap() == height + 1 {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn dfs_unique(
    grid: &RectangularGrid<u8>,
    memoize: &mut Fnv1aHashMap<(usize, usize), Fnv1aHashSet<(usize, usize)>>,
    x: usize,
    y: usize,
) -> Fnv1aHashSet<(usize, usize)> {
    if let Some(reachable) = memoize.get(&(x, y)) {
        return reachable.clone();
    }
    let height = *grid.at(x, y).unwrap();
    let neighbors = get_neighbors(grid, x, y, height);
    let mut reachable = Fnv1aHashSet::default();
    for (nx, ny) in neighbors {
        if height == 8 {
            reachable.insert((nx, ny));
        } else {
            for (x1, y1) in dfs_unique(grid, memoize, nx, ny) {
                reachable.insert((x1, y1));
            }
        }
    }
    memoize.insert((x, y), reachable.clone());
    reachable
}

fn dfs_all(
    grid: &RectangularGrid<u8>,
    memoize: &mut RectangularGrid<Option<usize>>,
    x: usize,
    y: usize,
) -> usize {
    if let Some(Some(count)) = memoize.at(x, y) {
        return *count;
    }
    let height = *grid.at(x, y).unwrap();
    let neighbors = get_neighbors(grid, x, y, height);
    let mut count = 0;
    for (nx, ny) in neighbors {
        if height == 8 {
            count += 1;
        } else {
            count += dfs_all(grid, memoize, nx, ny);
        }
    }
    memoize.set(x, y, Some(count));
    count
}

fn solve1(input: &str) -> usize {
    let grid = parse_grid(input);
    let heads = grid
        .iter()
        .filter(|(_, _, &c)| c == 0)
        .map(|(x, y, _)| (x, y))
        .collect::<Vec<_>>();
    heads
        .iter()
        .map(|&(x, y)| dfs_unique(&grid, &mut Fnv1aHashMap::default(), x, y).len())
        .sum()
}

fn solve2(input: &str) -> usize {
    let grid = parse_grid(input);
    let heads = grid
        .iter()
        .filter(|(_, _, &c)| c == 0)
        .map(|(x, y, _)| (x, y))
        .collect::<Vec<_>>();
    heads
        .iter()
        .map(|&(x, y)| {
            dfs_all(
                &grid,
                &mut RectangularGrid::new(grid.width, grid.height),
                x,
                y,
            )
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let fs = [solve1, solve2];
    solver::solver(fs, input);
}
