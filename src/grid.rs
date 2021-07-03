use grid_view::GridView;

use crate::util::{modulo, generate_seed};
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::cell::Cell;
pub use crate::grid::boundary::Boundary;

pub mod grid_pos;
pub mod grid_view;
mod boundary;

#[derive(Clone, Debug)]
pub struct Grid<C: Cell> {
    cells: Vec<C>,
    width: usize,
    height: usize,
    boundary: Boundary<C>,
    // Should always be the same size as `cells`. When updating, we read from
    // `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
    // use, and `cells` should be updated directly.
    scratch_cells: Vec<C>,
}

impl<C: Cell> Grid<C> {
    fn new_empty(width: usize, height: usize, boundary: Boundary<C>) -> Self {
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

    pub fn new_random(width: usize, height: usize, boundary: Boundary<C>) -> Self {
        let mut result = Self::new_empty(width, height, boundary);
        result.randomize();
        result
    }

    pub fn randomize(&mut self) {
        let mut rng: randomize::PCG32 = generate_seed().into();
        for c in self.cells.iter_mut() {
            *c = C::random(&mut rng);
        }
        // run a few simulation iterations for aesthetics (If we don't, the
        // noise is ugly)
        for _ in 0..3 {
            self.update();
        }
    }
    pub fn get_cell_at(&self, x: GridInt, y: GridInt) -> &C {
        let (width, height) = (self.width as GridInt, self.height as GridInt);
        match &self.boundary {
            Boundary::Periodic => {
                let cell_idx = modulo(x, width) + modulo(y, height) * width;
                &self.cells[cell_idx as usize]
            }
            Boundary::Constant(cell) => {
                if (0 <= x) & (x < width) & (0 <= y) & (y < height) {
                    let cell_idx = x + y * width;
                    &self.cells[cell_idx as usize]
                } else {
                    &cell
                }
            }
        }

    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = x + y * self.width;
                let grid_pos = GridPos::new(x as GridInt, y as GridInt);
                let grid_view = GridView::new(grid_pos, &self);
                let next = self.cells[idx].update(grid_view);
                // Write into scratch_cells, since we're still reading from `self.cells`
                self.scratch_cells[idx] = next;
            }
        }
        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }

    pub fn toggle(&mut self, x: isize, y: isize) -> bool {
        match self.grid_idx(x, y, false) {
            Some(i) => {
                self.cells[i].toggle();
                true
            }
            None => false
        }
    }

    pub fn draw(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.draw());
        }
    }

    pub fn set_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, alive: bool) {
        // probably should do sutherland-hodgeman if this were more serious.
        // instead just clamp the start pos, and draw until moving towards the
        // end pos takes us out of bounds.
        let x0 = x0.max(0).min(self.width as isize);
        let y0 = y0.max(0).min(self.height as isize);
        for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
            if let Some(i) = self.grid_idx(x, y, false) {
                Self::line_action(&mut self.cells[i], alive)
            } else {
                break;
            }
        }
    }
    fn line_action(cell: &mut C, alive: bool) {
        cell.line_action(alive)
    }
    fn grid_idx<>(&self, x: isize, y: isize, periodic: bool) -> Option<usize> {
        let (x, y) = (x as usize, y as usize);
        match self.boundary {
            Boundary::Periodic => Some((x % self.width) + (y % self.height) * self.width),
            Boundary::Constant(_) => {
                if x < self.width && y < self.height {
                    Some(x + y * self.width)
                } else {
                    None
                }
            }
        }
    }
}
