use crate::cell::Cell;
use crate::grid::grid_pos::{GridPos, GridInt};
use crate::util::modulo;
use crate::grid::Grid;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait BoundaryTrait<C : Cell> {
    fn grid_map<'a>(&'a self, grid_pos: &GridPos, grid: &'a Grid<C>) -> &'a C;
}

#[derive(Debug)]
pub struct PeriodicBoundary;
impl<C: Cell> BoundaryTrait<C> for PeriodicBoundary {
    fn grid_map<'a>(&'a self, grid_pos: &GridPos, grid: &'a Grid<C>) -> &'a C {
        let (width, height) = (grid.width as GridInt, grid.height as GridInt);
        let new_grid_pos = GridPos::new(
            modulo(grid_pos.x(), width),
            modulo(grid_pos.y(), height)
        );
        grid.raw_get_cell_at(&new_grid_pos).unwrap()
    }
}
#[derive(Debug)]
pub struct ConstantBoundary<C: Cell>(pub C);
impl<C: Cell> ConstantBoundary<C> {
    pub fn empty() -> Self {
        ConstantBoundary(C::default())
    }
}
impl<C: Cell> BoundaryTrait<C> for ConstantBoundary<C> {
    fn grid_map<'a>(&'a self, grid_pos: &GridPos, grid: &'a Grid<C>) -> &'a C {
        match grid.raw_get_cell_at(grid_pos) {
            Some(cell) => cell,
            None => &self.0
        }
    }
}

#[derive(Debug)]
pub struct FreeBoundary;
impl<C: Cell> BoundaryTrait<C> for FreeBoundary {
    fn grid_map<'a>(&'a self, grid_pos: &GridPos, grid: &'a Grid<C>) -> &'a C {
        let new_grid_pos = GridPos::new(
            grid_pos.x().clamp(0, (grid.width - 1) as GridInt),
            grid_pos.y().clamp(0, (grid.height - 1) as GridInt)
        );
        grid.raw_get_cell_at(&new_grid_pos).unwrap()
    }
}

#[enum_dispatch(BoundaryTrait<C>)]
#[derive(Debug)]
pub enum Boundary<C: Cell> {
    PeriodicBoundary,
    ConstantBoundary(ConstantBoundary<C>),
    FreeBoundary
}