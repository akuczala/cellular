use grid_view::GridView;

use crate::util::{generate_seed};
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::cell::Cell;
use crate::grid::boundary::{Boundary, BoundaryTrait};

pub mod grid_pos;
pub mod grid_view;
pub mod boundary;

#[derive(Debug)]
pub struct Grid<C: Cell> {
    pub cells: Vec<C>,
    pub width: usize,
    pub height: usize,
    boundary: Boundary<C>,
    // Should always be the same size as `cells`. When updating, we read from
    // `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
    // use, and `cells` should be updated directly.
    scratch_cells: Vec<C>,
}

impl<'a, C: Cell> Grid<C> {
    fn new_empty(width: usize, height: usize, boundary: Boundary<C>) -> Self {
        assert!(width != 0 && height != 0);
        let size = width.checked_mul(height).expect("too big");
        Self {
            cells: vec![C::default(); size],
            scratch_cells: vec![C::default(); size],
            width,
            height,
            boundary
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
    pub fn get_cell_at(&self, x: GridInt, y: GridInt) -> &C {
        self.boundary.grid_map(&GridPos::new(x, y), &self)
    }
    fn to_idx(&self, grid_pos: &GridPos) -> usize {
        (grid_pos.x() as usize + grid_pos.y() as usize * self.width)
    }
    fn get_grid_pos_iter(&self) -> impl Iterator<Item=GridPos> {
        let (width, height) = (self.width as GridInt, self.height as GridInt);
        (0..height)
            .map(move |y| (0..width)
                .map(move |x| GridPos::new(x as GridInt, y as GridInt))
            ).flatten()
    }

    pub fn update(&mut self) {
        for grid_pos in self.get_grid_pos_iter() {
            let idx = self.to_idx(&grid_pos);
            let grid_view = GridView::new(grid_pos, &self);
            let next = self.cells[idx].update(grid_view);
            // Write into scratch_cells, since we're still reading from `self.cells`
            self.scratch_cells[idx] = next;
        }
        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }

    pub fn toggle(&mut self, x: isize, y: isize) -> bool {
        match self.grid_idx(x, y) {
            Some(i) => {
                let target_pos = GridPos::new(x as GridInt, y as GridInt);
               for grid_pos in self.get_grid_pos_iter() {
                    let idx = self.to_idx(&grid_pos);
                    self.cells[idx].toggle(&target_pos, &grid_pos)
                }
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
            let target_pos = GridPos::new(x as GridInt, y as GridInt);
            if let Some(i) = self.grid_idx(x, y) {
                self.line_action(target_pos, alive)
            } else {
                break;
            }
        }
    }
    fn line_action(&mut self, target_pos: GridPos, alive: bool) {
        for grid_pos in self.get_grid_pos_iter() {
            let idx = self.to_idx(&grid_pos);
            self.cells[idx].line_action(&target_pos, &grid_pos, alive)
        }

    }
    pub fn raw_get_cell_at(&self, grid_pos: &GridPos) -> Option<&C> {
        self.grid_idx(grid_pos.x() as isize, grid_pos.y() as isize)
            .map(|idx| &self.cells[idx])
    }
    fn grid_idx(&self, x: isize, y: isize) -> Option<usize> {
        let (x, y) = (x as usize, y as usize);
        if x < self.width && y < self.height {
            Some(x + y * self.width)
        } else {
            None
        }
    }
}
