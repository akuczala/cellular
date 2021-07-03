use crate::grid::grid_view::GridView;
use crate::util::{Color, RandomGenerator};

pub trait Cell: Sized + Default + Clone {
    fn random(rng: &mut RandomGenerator) -> Self;
    fn update(&self, grid_view: GridView<Self>) -> Self;
    fn draw(&self) -> Color;
    fn toggle(&mut self);
    fn line_action(&mut self, alive: bool);
}