use grid_view::GridView;

use crate::cell::Cell;
use crate::grid::boundary::{Boundary, BoundaryTrait};
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::util::generate_seed;

pub mod boundary;
pub mod grid_pos;
pub mod grid_view;

#[derive(Debug)]
pub struct Grid<C: Cell> {
    pub cells: Vec<C>,
    pub width: usize,
    pub height: usize,
    boundary: Boundary<C>,
    // Should always be the same size as `cells`. When updating, we read from
    // `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
    // use, and `cells` should be updated directly.
    pub scratch_cells: Vec<C>,
}

impl<'a, C: Cell> Grid<C> {
    pub fn new_empty(width: usize, height: usize, boundary: Boundary<C>) -> Self {
        assert!(width != 0 && height != 0);
        let size = width.checked_mul(height).expect("too big");
        Self {
            cells: vec![C::default(); size],
            scratch_cells: vec![C::default(); size],
            width,
            height,
            boundary,
        }
    }

    pub fn clear(&mut self) {
        for c in self.cells.iter_mut() {
            *c = C::default();
        }
    }

    pub fn new_random(width: usize, height: usize, boundary: Boundary<C>) -> Self {
        let mut result = Self::new_empty(width, height, boundary);
        result.randomize();
        result
    }

    pub fn randomize(&mut self) {
        let mut rng: randomize::PCG32 = generate_seed().into();
        for grid_pos in self.get_grid_pos_iter() {
            let idx = self.to_idx(&grid_pos);
            self.cells[idx] = C::random(&mut rng, grid_pos);
        }
    }
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }
    pub fn get_cell_at(&self, grid_pos: GridPos) -> &C {
        self.boundary.grid_map(&grid_pos, &self)
    }
    pub fn set_scatch_cell_at(&mut self, grid_pos: GridPos, cell: C) {
        let idx = self.to_idx(&grid_pos);
        self.scratch_cells[idx] = cell;
    }
    // todo make private
    pub fn to_idx(&self, grid_pos: &GridPos) -> usize {
        grid_pos.x() as usize + grid_pos.y() as usize * self.width
    }
    pub fn get_grid_pos_iter(&self) -> impl Iterator<Item = GridPos> {
        let (width, height) = (self.width as GridInt, self.height as GridInt);
        (0..height)
            .map(move |y| (0..width).map(move |x| GridPos::new(x as GridInt, y as GridInt)))
            .flatten()
    }

    pub fn draw(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.draw());
        }
    }

    pub fn raw_get_cell_at(&self, grid_pos: &GridPos) -> Option<&C> {
        self.grid_idx(grid_pos.x() as isize, grid_pos.y() as isize)
            .map(|idx| &self.cells[idx])
    }
    pub fn grid_idx(&self, x: isize, y: isize) -> Option<usize> {
        let (x, y) = (x as usize, y as usize);
        if x < self.width && y < self.height {
            Some(x + y * self.width)
        } else {
            None
        }
    }
}
