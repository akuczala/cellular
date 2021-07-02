use crate::cell::Cell;
use crate::util::modulo;


const INITIAL_FILL: f32 = 0.3;
const NEIGHBOR_OFFSETS: [[i32; 2]; 8] = [[1,0],[1,1],[0,1],[-1,1],[-1,0],[-1,-1],[0,-1],[1,-1]];
#[derive(Clone, Debug)]
pub(crate) struct ConwayGrid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    // Should always be the same size as `cells`. When updating, we read from
    // `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
    // use, and `cells` should be updated directly.
    scratch_cells: Vec<Cell>,
}

impl ConwayGrid {
    fn new_empty(width: usize, height: usize) -> Self {
        assert!(width != 0 && height != 0);
        let size = width.checked_mul(height).expect("too big");
        Self {
            cells: vec![Cell::default(); size],
            scratch_cells: vec![Cell::default(); size],
            width,
            height,
        }
    }

    pub(crate) fn new_random(width: usize, height: usize) -> Self {
        let mut result = Self::new_empty(width, height);
        result.randomize();
        result
    }

    pub(crate) fn randomize(&mut self) {
        let mut rng: randomize::PCG32 = generate_seed().into();
        for c in self.cells.iter_mut() {
            let alive = randomize::f32_half_open_right(rng.next_u32()) > INITIAL_FILL;
            *c = Cell::new(alive);
        }
        // run a few simulation iterations for aesthetics (If we don't, the
        // noise is ugly)
        for _ in 0..3 {
            self.update();
        }
        // Smooth out noise in the heatmap that would remain for a while
        for c in self.cells.iter_mut() {
            c.cool_off(0.4);
        }
    }
    fn get_cell_at(&self, x: i32, y: i32) -> &Cell {
        let (width, height) = (self.width as i32, self.height as i32);
        let cell_idx = modulo(x, width) + modulo(y, height) * width;
        &self.cells[cell_idx as usize]
    }
    fn count_neibs(&self, ux: usize, uy: usize) -> usize {
        let (x, y) = (ux as i32, uy as i32);
        NEIGHBOR_OFFSETS
            .iter()
            .map(|dxy| { self.get_cell_at(x + dxy[0], y + dxy[1]).alive as usize })
            .sum()
    }
    pub(crate) fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let neibs = self.count_neibs(x, y);
                let idx = x + y * self.width;
                let next = self.cells[idx].update_neibs(neibs);
                // Write into scratch_cells, since we're still reading from `self.cells`
                self.scratch_cells[idx] = next;
            }
        }
        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }

    pub(crate) fn toggle(&mut self, x: isize, y: isize) -> bool {
        if let Some(i) = self.grid_idx(x, y, false) {
            let was_alive = self.cells[i].alive;
            self.cells[i].set_alive(!was_alive);
            !was_alive
        } else {
            false
        }
    }

    pub(crate) fn draw(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            let color = if c.alive {
                [0, 0xff, 0xff, 0xff]
            } else {
                [0, 0, c.heat, 0xff]
            };
            pix.copy_from_slice(&color);
        }
    }

    pub(crate) fn set_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, alive: bool) {
        // probably should do sutherland-hodgeman if this were more serious.
        // instead just clamp the start pos, and draw until moving towards the
        // end pos takes us out of bounds.
        let x0 = x0.max(0).min(self.width as isize);
        let y0 = y0.max(0).min(self.height as isize);
        for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
            if let Some(i) = self.grid_idx(x, y, false) {
                self.cells[i].set_alive(alive);
            } else {
                break;
            }
        }
    }
    // todo make this work for int properly
    fn grid_idx<I: std::convert::TryInto<usize>>(&self, x: I, y: I, periodic: bool) -> Option<usize> {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            match periodic {
                true => Some((x % self.width) + (y % self.height) * self.width),
                false => {
                    if x < self.width && y < self.height {
                        Some(x + y * self.width)
                    } else {
                        None
                    }
                }
            }

        } else {
            None
        }
    }
}

/// Generate a pseudorandom seed for the game's PRNG.
fn generate_seed() -> (u64, u64) {
    use byteorder::{ByteOrder, NativeEndian};
    use getrandom::getrandom;

    let mut seed = [0_u8; 16];

    getrandom(&mut seed).expect("failed to getrandom");

    (
        NativeEndian::read_u64(&seed[0..8]),
        NativeEndian::read_u64(&seed[8..16]),
    )
}