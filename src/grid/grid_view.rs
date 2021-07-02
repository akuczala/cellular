use crate::grid::ConwayGrid;
use crate::cell::Cell;
use crate::grid::grid_pos::{GridPos, GridInt};

pub struct GridView<'a> {
    origin: GridPos,
    grid: &'a ConwayGrid
}
impl<'a> GridView<'a> {
    pub fn new(origin: GridPos, grid: &ConwayGrid) -> GridView {
        GridView{origin, grid}
    }
    pub fn get_cell_at(&self, x: GridInt, y: GridInt) -> &Cell {
        self.grid.get_cell_at(self.origin.x() + x, self.origin.y() + y)
    }
}