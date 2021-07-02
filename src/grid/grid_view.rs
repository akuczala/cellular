use crate::grid::Grid;
use crate::grid::grid_pos::{GridPos, GridInt};
use crate::cell::Cell;

pub struct GridView<'a, C: Cell> {
    origin: GridPos,
    grid: &'a Grid<C>
}
impl<'a, C: Cell> GridView<'a, C> {
    pub fn new(origin: GridPos, grid: &Grid<C>) -> GridView<C> {
        GridView{origin, grid}
    }
    pub fn get_cell_at(&self, x: GridInt, y: GridInt) -> &C {
        self.grid.get_cell_at(self.origin.x() + x, self.origin.y() + y)
    }
}