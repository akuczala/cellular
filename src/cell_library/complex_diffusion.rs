use num_complex::Complex32;
use crate::cell::Cell;
use crate::util::{RandomGenerator, NEAREST_NEIGHBORS, N_NEAREST_NEIGHBORS, Color, complex_to_hue, SECOND_ORDER_CENTRAL, stencil_coords};
use crate::grid::grid_view::GridView;
use palette::{Hsv, LinSrgb, Pixel};
use std::f32::consts::PI;

type Density = Complex32;
type Float = f32;
const MAX_ABS: f32 = 1.0;
const DIFFUSION_CONSTANT: Float = 0.01;

#[derive(Default,Clone)]
pub struct ComplexDiffusionCell {
    pub(crate) density: Density
}
impl ComplexDiffusionCell {
    fn avg_neighbors(grid_view: GridView<Self>) -> Density {
        NEAREST_NEIGHBORS
            .iter()
            .map(|dxy| { grid_view.get_cell_at_coord(dxy[0], dxy[1]).density})
            .sum::<Density>() / (N_NEAREST_NEIGHBORS as Float)
    }
    fn laplace(grid_view: GridView<Self>) -> Density {
        let laplacian: Density = SECOND_ORDER_CENTRAL.iter()
            .flatten()
            .zip(stencil_coords(3, 3))
            .map(|(weight, dpos)| weight * grid_view.get_cell_at(dpos).density)
            .sum();
        laplacian*DIFFUSION_CONSTANT / Density::i() + grid_view.get_cell_at_coord(0, 0).density * (1.0 - DIFFUSION_CONSTANT)
    }
}

impl Cell for ComplexDiffusionCell {
    fn random(rng: &mut RandomGenerator) -> Self {
        let radius = randomize::f32_half_open_right(rng.next_u32());
        let theta = randomize::f32_half_open_right(rng.next_u32()) * 2.0 * PI;
        Self{density: Density::from_polar(radius, theta)}
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        let new_density: Density = Self::laplace(grid_view);
        let new_density = new_density / new_density.norm().clamp(MAX_ABS, Float::INFINITY);
        Self{density: new_density}
    }

    fn draw(&self) -> Color {
        let hue = complex_to_hue(self.density);
        let value = self.density.norm();
        let rgb: [u8; 3] = LinSrgb::from(Hsv::new(hue, 1.0, value)).into_format().into_raw();
        [rgb[0], rgb[1], rgb[2], 0]
    }

    fn toggle(&mut self) {

    }

    fn line_action(&mut self, alive: bool) {
        self.density = Density::new(MAX_ABS, 0.0);
    }
}

#[test]
fn test_draw() {
    let cell = ComplexDiffusionCell{density: Density::new(-1.0, 0.0)};
    let hue = complex_to_hue(cell.density);
    let value = cell.density.norm();
    //assert_eq!(hue, 0.5);
    assert_eq!(value, 1.0);
    let hsv = Hsv::new(hue, 1.0, value);
    println!("{:?}", hsv);
    println!("{:?}",Rgb::from(hsv));
    assert_eq!(cell.draw(), [0, 0xff, 0xff, 0]);

    let cell = ComplexDiffusionCell{density: Density::new(0.0, 1.0)};
    let hue = complex_to_hue(cell.density);
    let value = cell.density.norm();
    assert_eq!(hue, 0.75);
    assert_eq!(value, 1.0);
    println!("{:?}",Rgb::from(Hsv::new(hue, 1.0, value)));
    assert_eq!(cell.draw(), [0, 0xff, 0xff, 0]);
}