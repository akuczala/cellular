use crate::grid::grid_pos::{GridInt, GridPos};
use crate::grid::grid_view::GridView;
use crate::grid::Grid;
use crate::util::{Color, RandomGenerator};

// TODO: deprecate this oopy shit
pub trait Cell: Default + Clone {
    fn update(&self, grid_view: GridView<Self>) -> Self;
    fn draw(&self) -> Color;
    // todo make toggle and line action return new cells
    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos);
    fn line_action(&mut self, target_pos: &GridPos, grid_pos: &GridPos, alive: bool);
    // fn aggregate(&self) ->
}

pub trait Randomize {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self;
}
pub trait HasColor {
    fn draw(&self) -> Color;
}

pub trait System<C: Cell> {
    fn update(&mut self);
    fn update_cell(&self, grid_view: GridView<C>, cell: &C) -> C;
    fn get_grid(&self) -> &Grid<C>;
    fn get_grid_mut(&mut self) -> &mut Grid<C>;
    fn toggle(&mut self, x: isize, y: isize) -> bool;
    fn line_action(&mut self, target_pos: GridPos, alive: bool);
    fn set_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, alive: bool) {
        // probably should do sutherland-hodgeman if this were more serious.
        // instead just clamp the start pos, and draw until moving towards the
        // end pos takes us out of bounds.
        let x0 = x0.max(0).min(self.get_grid().width as isize);
        let y0 = y0.max(0).min(self.get_grid().height as isize);
        for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
            let target_pos = GridPos::new(x as GridInt, y as GridInt);
            if let Some(_i) = self.get_grid().grid_idx(x, y) {
                self.line_action(target_pos, alive)
            } else {
                break;
            }
        }
    }
}
