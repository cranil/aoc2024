use std::collections::{HashMap, HashSet, VecDeque};

struct Input {
    orderings: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let mut orderings = HashSet::new();
    let mut updates = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split("|");
        let first = parts.next().unwrap().parse::<usize>().unwrap();
        let second = parts.next().unwrap().parse::<usize>().unwrap();
        let ordering = (first, second);
        orderings.insert(ordering);
    }
    for line in lines {
        let page = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        updates.push(page);
    }

    Input { orderings, updates }
}

fn check(orderings: &HashSet<(usize, usize)>, pages: &[usize]) -> bool {
    for ordering in orderings {
        let first = pages.iter().position(|&page| page == ordering.0);
        let second = pages.iter().position(|&page| page == ordering.1);
        if first.is_none() || second.is_none() {
            continue;
        }
        let first = first.unwrap();
        let second = second.unwrap();
        if first < second {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn solve1(input: &Input) -> usize {
    let Input { orderings, updates } = input;
    let mut sum = 0;
    updates.iter().for_each(|pages| {
        let valid = check(orderings, pages);
        if valid {
            sum += pages[pages.len() / 2];
        }
    });
    sum
}

fn topological_sort(pages: &HashSet<usize>, orderings: &HashSet<(usize, usize)>) -> Vec<usize> {
    let mut neighbours = pages
        .iter()
        .map(|&x| (x, HashSet::new()))
        .collect::<HashMap<_, _>>();
    let mut edges = orderings
        .iter()
        .filter_map(|&(from, to)| {
            if !pages.contains(&from) || !pages.contains(&to) {
                None
            } else {
                neighbours.get_mut(&from).unwrap().insert(to);
                Some((from, to))
            }
        })
        .collect::<HashSet<_>>();
    let mut indegrees = pages.iter().map(|&x| (x, 0)).collect::<HashMap<_, _>>();
    edges.iter().for_each(|edge| {
        let e = indegrees.entry(edge.1).or_insert(0);
        *e += 1;
    });

    let mut s = indegrees
        .iter()
        .filter_map(|(i, d)| if *d == 0 { Some(*i) } else { None })
        .collect::<VecDeque<_>>();
    let mut sorted = Vec::new();
    while let Some(node) = s.pop_front() {
        sorted.push(node);
        for neighbour in neighbours.get(&node).unwrap() {
            let e = indegrees.get_mut(neighbour).unwrap();
            *e -= 1;
            edges.remove(&(node, *neighbour));
            if *e == 0 {
                s.push_back(*neighbour);
            }
        }
    }
    if edges.is_empty() {
        sorted
    } else {
        panic!("cycle detected");
    }
}

fn solve2(input: &Input) -> usize {
    let mut sum = 0;
    for pages in &input.updates {
        if check(&input.orderings, pages) {
            continue;
        }
        let mut pages = pages.clone();
        pages.sort_by(|a, b| {
            if input.orderings.contains(&(*a, *b)) {
                std::cmp::Ordering::Less
            } else if input.orderings.contains(&(*b, *a)) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        sum += pages[pages.len() / 2];
    }
    sum
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input);
    let fs = [solve1, solve2];
    solver::solver(fs, &input);
}
