use crate::cell::{Cell, System};
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::grid::grid_view::GridView;
use crate::grid::Grid;

pub struct GenericSystemData(pub i32);

pub struct GenericSystem<C: Cell> {
    pub grid: Grid<C>,
}
impl<C: Cell> GenericSystem<C> {
    pub fn new(grid: Grid<C>) -> Self {
        Self { grid }
    }
}
impl<C: Cell> System<C> for GenericSystem<C> {
    fn update(&mut self) {
        for grid_pos in self.get_grid().get_grid_pos_iter() {
            let grid_view = GridView::new(grid_pos, &self.get_grid());
            let cell = self.get_grid().get_cell_at(grid_pos);
            let next = self.update_cell(grid_view, cell);
            // Write into scratch_cells, since we're still reading from `self.cells`
            self.get_grid_mut().set_scatch_cell_at(grid_pos, next);
        }
        self.get_grid_mut().swap()
    }
    fn update_cell(&self, grid_view: GridView<C>, cell: &C) -> C {
        cell.update(grid_view)
    }

    fn get_grid(&self) -> &Grid<C> {
        &self.grid
    }

    fn get_grid_mut(&mut self) -> &mut Grid<C> {
        &mut self.grid
    }

    fn toggle(&mut self, x: isize, y: isize) -> bool {
        match self.grid.grid_idx(x, y) {
            Some(i) => {
                let target_pos = GridPos::new(x as GridInt, y as GridInt);
                for grid_pos in self.grid.get_grid_pos_iter() {
                    let idx = self.grid.to_idx(&grid_pos);
                    self.grid.cells[idx].toggle(&target_pos, &grid_pos)
                }
                true
            }
            None => false,
        }
    }
    fn line_action(&mut self, target_pos: GridPos, alive: bool) {
        for grid_pos in self.grid.get_grid_pos_iter() {
            let idx = self.grid.to_idx(&grid_pos);
            self.grid.cells[idx].line_action(&target_pos, &grid_pos, alive)
        }
    }
}
