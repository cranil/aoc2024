use std::cell::Cell;

pub mod adj_list;
pub mod algos;
pub mod grid_graph;

pub enum Error {
    DuplicateEntry,
    OutOfMemory,
    IdNotFound(usize),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DuplicateEntry => write!(f, "Duplicate entry"),
            Error::OutOfMemory => write!(f, "Out of memory"),
            Error::IdNotFound(id) => write!(f, "Id not found: {}", id),
        }
    }
}

impl From<Error> for Result<(), Error> {
    fn from(e: Error) -> Result<(), Error> {
        Err(e)
    }
}

pub struct WrappedNode<'a, Content: 'a> {
    pub id: usize,
    pub content: &'a Content,
}

pub struct WrappedNodeMut<'a, Content: 'a> {
    pub id: usize,
    pub content: &'a mut Content,
}

pub struct WrappedEdge {
    pub edge: (usize, usize),
}

pub trait GraphBuilder<'a, Content> {
    fn insert(&mut self, node: Content) -> Result<(), Error>;
    fn connect(&mut self, from: usize, to: usize) -> Result<(), Error>;
}

pub trait Graph<'a, Content> {
    fn node(&'a self, id: usize) -> Option<WrappedNode<'a, Content>> {
        self.nodes().find(|n| n.id == id)
    }

    fn nodes(&'a self) -> impl Iterator<Item = WrappedNode<'a, Content>>
    where
        Content: 'a;

    fn edges(&'a self) -> impl Iterator<Item = WrappedEdge>
    where
        Content: 'a;

    fn neighbours(&self, id: usize) -> impl Iterator<Item = usize>;
}

pub trait GraphMut<'a, Content: 'a>: Graph<'a, Cell<Content>> {
    fn node_mut(&'a mut self, id: usize) -> Option<WrappedNodeMut<'a, Content>> {
        self.nodes_mut().find(|n| n.id == id)
    }

    fn nodes_mut(&'a mut self) -> impl Iterator<Item = WrappedNodeMut<'a, Content>>
    where
        Content: 'a;
}

pub trait WeightedGraph<'a, Content, Weight: Clone>: Graph<'a, Content> {
    fn weight(&self, edge: (usize, usize)) -> Option<Weight>;
}

pub trait WeightedGraphMut<'a, Content, Weight: Clone>: WeightedGraph<'a, Content, Weight> {
    fn set_weight(&mut self, edge: (usize, usize), weight: Weight) -> Result<(), Error>;
}

pub trait WeightedGraphBuilder<'a, Content, Weight: Clone> {
    fn insert(&mut self, node: Content) -> Result<(), Error>;
    fn connect(&mut self, from: usize, to: usize, weight: Weight) -> Result<(), Error>;
}
