use crate::grid::grid_view::GridView;
use crate::util::{Color, RandomGenerator};
use crate::grid::grid_pos::GridPos;

pub trait Cell: Default + Clone {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self;
    fn update(&self, grid_view: GridView<Self>) -> Self;
    fn draw(&self) -> Color;
    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos);
    fn line_action(&mut self, target_pos: &GridPos, grid_pos: &GridPos, alive: bool);
    // fn aggregate(&self) ->
}