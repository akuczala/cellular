use crate::cell::Cell;
use crate::grid::grid_view::GridView;
use crate::util::{RandomGenerator, NEAREST_NEIGHBORS, N_NEAREST_NEIGHBORS, Color, SECOND_ORDER_CENTRAL, stencil_coords, map_to_unit_interval};

type Density = f32;
const MIN_VISIBLE_DENSITY: Density = 0.0;
const MAX_VISIBLE_DENSITY: Density = 1.0;
const DIFFUSION_CONSTANT: Density = 0.001;

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
        let laplacian: Density = SECOND_ORDER_CENTRAL.iter()
            .flatten()
            .zip(stencil_coords(3, 3))
            .map(|(weight, dpos)| weight * grid_view.get_cell_at(dpos).density)
            .sum();
        laplacian*DIFFUSION_CONSTANT + grid_view.get_cell_at_coord(0, 0).density * (1.0 - DIFFUSION_CONSTANT)
    }
}
impl Cell for DiffusionCell {
    fn random(rng: &mut RandomGenerator) -> Self {
        Self{density: randomize::f32_half_open_right(rng.next_u32())}
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        let new_density: Density = Self::laplace(grid_view) ;
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

    fn toggle(&mut self) {

    }

    fn line_action(&mut self, alive: bool) {
        self.density = MAX_VISIBLE_DENSITY;
    }
}