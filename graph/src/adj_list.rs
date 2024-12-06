use crate::{Graph, GraphBuilder, GraphMut, WeightedGraph, WeightedGraphBuilder};
use std::{cell::Cell, collections::HashMap};

pub struct AdjacencyList<T> {
    pub nodes: HashMap<usize, T>,
    pub edges: HashMap<usize, Vec<usize>>,
}

impl<T> AdjacencyList<T> {
    pub fn new() -> Self {
        AdjacencyList {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<T> Default for AdjacencyList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: 'a> Graph<'a, T> for AdjacencyList<T> {
    fn nodes(&'a self) -> impl std::iter::Iterator<Item = super::WrappedNode<'a, T>> {
        let it = self
            .nodes
            .iter()
            .map(|(id, content)| super::WrappedNode { id: *id, content });
        it
    }

    fn edges(&'a self) -> impl std::iter::Iterator<Item = super::WrappedEdge> {
        self.edges.iter().flat_map(move |(from, to)| {
            to.iter()
                .map(move |to| crate::WrappedEdge { edge: (*from, *to) })
        })
    }

    fn neighbours(&self, id: usize) -> impl Iterator<Item = usize> {
        self.edges.get(&id).unwrap().iter().copied()
    }
}

impl<T> GraphBuilder<'_, T> for AdjacencyList<T> {
    fn insert(&mut self, node: T) -> Result<(), super::Error> {
        let id = loop {
            let id = rand::random::<usize>();
            if !self.nodes.contains_key(&id) {
                break id;
            } else {
                continue;
            }
        };
        self.nodes.insert(id, node);
        self.edges.insert(id, Vec::new());
        Ok(())
    }

    fn connect(&mut self, from: usize, to: usize) -> Result<(), super::Error> {
        if !self.nodes.contains_key(&from) {
            return super::Error::IdNotFound(from).into();
        }
        if !self.nodes.contains_key(&to) {
            return super::Error::IdNotFound(to).into();
        }
        self.edges.get_mut(&from).unwrap().push(to);
        Ok(())
    }
}

impl<'a, T: 'a> GraphMut<'a, T> for AdjacencyList<Cell<T>> {
    fn node_mut(&'a mut self, id: usize) -> Option<crate::WrappedNodeMut<'a, T>> {
        self.nodes
            .get_mut(&id)
            .map(|content| crate::WrappedNodeMut {
                id,
                content: content.get_mut(),
            })
    }

    fn nodes_mut(&'a mut self) -> impl Iterator<Item = crate::WrappedNodeMut<'a, T>> {
        self.nodes
            .iter_mut()
            .map(|(id, content)| crate::WrappedNodeMut {
                id: *id,
                content: content.get_mut(),
            })
    }
}

pub struct WeightedAdjacencyList<T, W> {
    nodes: HashMap<usize, T>,
    edges: HashMap<usize, Vec<(usize, W)>>,
}

impl<T, W> WeightedAdjacencyList<T, W> {
    fn new() -> Self {
        WeightedAdjacencyList {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<T, W> Default for WeightedAdjacencyList<T, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, W: Clone> Graph<'a, T> for WeightedAdjacencyList<T, W> {
    fn nodes(&'a self) -> impl std::iter::Iterator<Item = super::WrappedNode<'a, T>> {
        let it = self
            .nodes
            .iter()
            .map(|(id, content)| super::WrappedNode { id: *id, content });
        it
    }

    fn edges(&'a self) -> impl std::iter::Iterator<Item = super::WrappedEdge> {
        self.edges.iter().flat_map(move |(from, to)| {
            to.iter()
                .map(move |(to, _)| crate::WrappedEdge { edge: (*from, *to) })
        })
    }

    fn neighbours(&self, id: usize) -> impl Iterator<Item = usize> {
        self.edges.get(&id).unwrap().iter().map(|(to, _)| *to)
    }
}

impl<T, W: Clone> WeightedGraphBuilder<'_, T, W> for WeightedAdjacencyList<T, W> {
    fn insert(&mut self, node: T) -> Result<(), super::Error> {
        let id = loop {
            let id = rand::random::<usize>();
            if !self.nodes.contains_key(&id) {
                break id;
            } else {
                continue;
            }
        };
        self.nodes.insert(id, node);
        self.edges.insert(id, Vec::new());
        Ok(())
    }

    fn connect(&mut self, from: usize, to: usize, weight: W) -> Result<(), super::Error> {
        if !self.nodes.contains_key(&from) {
            return super::Error::IdNotFound(from).into();
        }
        if !self.nodes.contains_key(&to) {
            return super::Error::IdNotFound(to).into();
        }
        self.edges
            .get_mut(&from)
            .unwrap()
            .push((to, weight.clone()));
        self.edges
            .get_mut(&to)
            .unwrap()
            .push((from, weight.clone()));
        Ok(())
    }
}

impl<T, W: Clone> WeightedGraph<'_, T, W> for WeightedAdjacencyList<T, W> {
    fn weight(&self, edge: (usize, usize)) -> Option<W> {
        self.edges
            .get(&edge.0)
            .unwrap()
            .iter()
            .find_map(|(to, w)| if *to == edge.1 { Some(w.clone()) } else { None })
    }
}

#[cfg(test)]
mod test {
    use super::AdjacencyList;
    use crate::{
        Graph, GraphBuilder,
        algos::{bfs, dfs},
    };

    #[test]
    fn test_bfs() {
        let mut graph = AdjacencyList::new();
        graph.insert(0).unwrap();
        graph.insert(1).unwrap();
        graph.insert(2).unwrap();
        graph.insert(3).unwrap();
        graph.insert(4).unwrap();
        let node_ids = graph.nodes().map(|n| n.id).collect::<Vec<_>>();
        println!("{:?}", node_ids);
        println!("{:?}", graph.edges.iter().collect::<Vec<_>>());
        graph.connect(node_ids[0], node_ids[1]).unwrap();
        graph.connect(node_ids[0], node_ids[2]).unwrap();
        graph.connect(node_ids[1], node_ids[2]).unwrap();
        graph.connect(node_ids[1], node_ids[3]).unwrap();
        graph.connect(node_ids[2], node_ids[4]).unwrap();

        for node in graph.nodes() {
            let nbrs = graph.neighbours(node.id).collect::<Vec<_>>();
            println!("{}: {:?}", node.id, nbrs);
        }

        let bfs = bfs(&graph, node_ids[0]);
        let bfs: Vec<_> = bfs.collect();
        println!("{:?}", bfs);
        assert_eq!(bfs.len(), 5);
    }

    #[test]
    fn test_dfs() {
        let mut graph = AdjacencyList::default();
        graph.insert(0).unwrap();
        graph.insert(1).unwrap();
        graph.insert(2).unwrap();
        graph.insert(3).unwrap();
        graph.insert(4).unwrap();
        let node_ids = graph.nodes().map(|n| n.id).collect::<Vec<_>>();
        println!("{:?}", node_ids);
        println!("{:?}", graph.edges.iter().collect::<Vec<_>>());
        graph.connect(node_ids[0], node_ids[1]).unwrap();
        graph.connect(node_ids[0], node_ids[2]).unwrap();
        graph.connect(node_ids[1], node_ids[2]).unwrap();
        graph.connect(node_ids[1], node_ids[3]).unwrap();
        graph.connect(node_ids[2], node_ids[4]).unwrap();

        for node in graph.nodes() {
            let nbrs = graph
                .neighbours(node.id)
                .map(|id| (id, graph.node(id).unwrap().content))
                .collect::<Vec<_>>();
            println!("{}: {:?}", node.id, nbrs);
        }

        let dfs = dfs(&graph, node_ids[0]);
        let dfs: Vec<_> = dfs.collect();
        println!("{:?}", dfs);
        assert_eq!(dfs.len(), 5);
    }
}
