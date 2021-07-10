use crate::cell::Cell;
use crate::grid::grid_view::GridView;
use crate::util::{RandomGenerator, NEAREST_NEIGHBORS, N_NEAREST_NEIGHBORS, Color, SECOND_ORDER_CENTRAL, stencil_coords, map_to_unit_interval, gauss};
use crate::grid::grid_pos::GridPos;

type Density = f32;
const MIN_VISIBLE_DENSITY: Density = 0.0;
const MAX_VISIBLE_DENSITY: Density = 1.0;
const DIFFUSION_CONSTANT: Density = 0.01;

#[derive(Clone,Default)]
pub struct DiffusionCell {
    pub density: Density
}
impl DiffusionCell {
    fn avg_neighbors(grid_view: GridView<Self>) -> Density {
        NEAREST_NEIGHBORS
            .iter()
            .map(|dxy| { grid_view.get_cell_at_coord(dxy[0], dxy[1]).density})
            .sum::<Density>() / (N_NEAREST_NEIGHBORS as Density)
    }
    fn laplace(grid_view: GridView<Self>) -> Density {
        SECOND_ORDER_CENTRAL.iter()
            .flatten()
            .zip(stencil_coords(3, 3))
            .map(|(weight, dpos)| weight * grid_view.get_cell_at(dpos).density)
            .sum()
    }
}
impl Cell for DiffusionCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        let density = if grid_pos.x() > 50 {
            randomize::f32_half_open_right(rng.next_u32()) * 1.0
        } else {
            0.0
        };
        Self{density}
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        let new_density: Density = Self::laplace(grid_view)*DIFFUSION_CONSTANT + self.density ;
        Self{density: new_density}
    }

    fn draw(&self) -> Color {
        let frac = map_to_unit_interval(
            self.density, MIN_VISIBLE_DENSITY, MAX_VISIBLE_DENSITY
        ).clamp(0.0, 1.0);
        let shade = (frac * (0xff as Density)) as u8;
        let shade2 = ((-4.0 * frac.powi(2) + 4.0 * frac) * (0xff as Density)) as u8;
        let shade3 = (frac.powi(3) * (0xff as Density)) as u8;
        [shade, shade, shade, 0]
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {

    }

    fn line_action(&mut self, target_pos: &GridPos, grid_pos: &GridPos, alive: bool) {
        self.density += gauss(1.0, 20.0, &target_pos, &grid_pos)
    }
}