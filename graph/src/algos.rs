use crate::{Graph, WeightedGraph};
use algos::binary_heap::BinaryHeap;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

pub struct DFSIterator<'a, Content, G: Graph<'a, Content>> {
    graph: &'a G,
    visited: HashSet<usize>,
    stack: Vec<usize>,
    phantom: std::marker::PhantomData<Content>, // to make the compiler happy
}

impl<'a, Content: 'a, G: Graph<'a, Content>> Iterator
    for DFSIterator<'a, Content, G>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            if self.visited.contains(&current) {
                continue;
            }
            self.visited.insert(current);
            self.stack.extend(self.graph.neighbours(current));
            return Some(current);
        }
        None
    }
}

pub fn dfs<'a, Content, G: Graph<'a, Content>>(
    graph: &'a G,
    start: usize,
) -> impl Iterator<Item = usize> + 'a
where
    Content: Sized + 'a,
{
    DFSIterator {
        graph,
        visited: HashSet::new(),
        stack: vec![start],
        phantom: std::marker::PhantomData,
    }
}

pub struct BFSIterator<'a, Content, G: Graph<'a, Content>> {
    graph: &'a G,
    visited: HashSet<usize>,
    queue: VecDeque<usize>,
    phantom: std::marker::PhantomData<Content>, // to make the compiler happy
}

impl<'a, Content: 'a, G: Graph<'a, Content>> Iterator
    for BFSIterator<'a, Content, G>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.queue.pop_front() {
            if self.visited.contains(&current) {
                continue;
            }
            self.visited.insert(current);
            self.queue.extend(self.graph.neighbours(current));
            return Some(current);
        }
        None
    }
}

pub fn bfs<'a, Content, G: Graph<'a, Content>>(
    graph: &'a G,
    start: usize,
) -> impl Iterator<Item = usize> + 'a
where
    Content: Sized + 'a,
{
    BFSIterator {
        graph,
        visited: HashSet::new(),
        queue: VecDeque::from(vec![start]),
        phantom: std::marker::PhantomData,
    }
}

pub struct DijkstraIterator<
    'a,
    Content,
    W: Clone + Ord + Add,
    G: WeightedGraph<'a, Content, W>,
> {
    graph: &'a G,
    visited: HashSet<usize>,
    queue: BinaryHeap<usize, W>,
    phantom: std::marker::PhantomData<Content>, // to make the compiler happy
}

impl<'a, Content: 'a, W: Clone + Ord + Add, G: WeightedGraph<'a, Content, W>>
    Iterator for DijkstraIterator<'a, Content, W, G>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current, _)) = self.queue.pop() {
            if self.visited.contains(&current) {
                continue;
            }
            self.visited.insert(current);
            for neighbour in self.graph.neighbours(current) {
                let weight =
                    self.graph.weight((current, neighbour)).unwrap().clone();
                self.queue.push(neighbour, weight);
            }
            return Some(current);
        }
        None
    }
}

pub fn dijkstra<
    'a,
    Content,
    W: Default + Clone + Ord + Add<Output = W>,
    G: WeightedGraph<'a, Content, W>,
>(
    graph: &'a G,
    start: usize,
) -> (W, Vec<usize>)
where
    Content: Sized + 'a,
{
    let mut dist = HashMap::new();
    let mut visited = HashSet::new();
    let mut parent = Vec::new();

    dist.insert(start, W::default());
    let mut queue = BinaryHeap::new();
    queue.push(start, W::default());
    while let Some((current, _)) = queue.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        for neighbour in graph.neighbours(current) {
            let weight = graph.weight((current, neighbour)).unwrap();
            let Some(current_dist) = dist.get(&current) else {
                eprintln!("This should not happen");
                continue;
            };
            let Some(neighbour_dist) = dist.get(&neighbour) else {
                let new_dist = current_dist.clone() + weight.clone();
                dist.insert(neighbour, new_dist.clone());
                queue.push(neighbour, new_dist);
                continue;
            };
            if current_dist.clone() + weight.clone() < neighbour_dist.clone() {
                let new_dist = current_dist.clone() + weight.clone();
                dist.insert(neighbour, new_dist.clone());
                queue.push(neighbour, new_dist);
            }
        }
    }
    (W::default(), parent)
}
