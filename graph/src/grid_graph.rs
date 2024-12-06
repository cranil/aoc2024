use algos::grid::Grid;

use crate::{Graph, WrappedNode};

struct GridGraph<'a, T: 'a, GridType: Grid<'a, T>> {
    pub grid: GridType,
    phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T, GridType: Grid<'a, T>> GridGraph<'a, T, GridType> {
    pub fn new(grid: GridType) -> Self {
        Self {
            grid,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T, GridType> From<GridType> for GridGraph<'a, T, GridType>
where
    GridType: Grid<'a, T>,
{
    fn from(grid: GridType) -> Self {
        Self::new(grid)
    }
}

// impl<'a, T: 'a, GridType: Grid<'a, T>> Graph<'a, T> for GridGraph<'a, T, GridType> {
//     fn nodes(&'a self) -> impl Iterator<Item = crate::WrappedNode<'a, T>>
//     where
//         T: 'a,
//     {
//         let (width, _) = self.grid.size();
//         self.grid.iter().map(move |(x, y, content)| WrappedNode {
//             id: x + y * width,
//             content,
//         })
//     }

//     fn edges(&'a self) -> impl Iterator<Item = crate::WrappedEdge>
//     where
//         T: 'a,
//     {
//         panic!()
//     }

//     fn neighbours(&self, id: usize) -> impl Iterator<Item = usize> {
//         panic!()
//     }
// }
