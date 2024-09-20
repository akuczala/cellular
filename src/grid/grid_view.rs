use crate::cell::Cell;
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::grid::Grid;

pub struct GridView<'a, C: Cell> {
    pub origin: GridPos,
    grid: &'a Grid<C>,
}
impl<'a, C: Cell> GridView<'a, C> {
    pub fn new(origin: GridPos, grid: &Grid<C>) -> GridView<C> {
        GridView { origin, grid }
    }
    pub fn get_cell_at_coord(&self, x: GridInt, y: GridInt) -> &C {
        self.grid
            .get_cell_at(GridPos::new(x + self.origin.x, y + self.origin.y))
    }
    pub fn get_cell_at(&self, pos: GridPos) -> &C {
        self.get_cell_at_coord(pos.x, pos.y)
    }
    pub fn grid_width(&self) -> GridInt {
        self.grid.width as GridInt
    }
    pub fn grid_height(&self) -> GridInt {
        self.grid.height as GridInt
    }
}
