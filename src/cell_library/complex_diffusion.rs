use num_complex::{Complex32, Complex64};
use crate::cell::Cell;
use crate::util::{RandomGenerator, NEAREST_NEIGHBORS, N_NEAREST_NEIGHBORS, Color, complex_to_hue, SECOND_ORDER_CENTRAL, stencil_coords};
use crate::grid::grid_view::GridView;
use palette::{Hsv, LinSrgb, Pixel};
use std::f32::consts::PI;
use crate::grid::grid_pos::GridPos;

type Density = Complex64;
type Float = f64;
const MAX_ABS: Float = 1.0;

#[derive(Default,Clone)]
pub struct ComplexDiffusionCell {
    pub(crate) density: Density
}
impl ComplexDiffusionCell {
    fn diffusion_constant() -> Density {
        0.001*Density::i()
    }
    fn avg_neighbors(grid_view: GridView<Self>) -> Density {
        NEAREST_NEIGHBORS
            .iter()
            .map(|dxy| { grid_view.get_cell_at_coord(dxy[0], dxy[1]).density})
            .sum::<Density>() / (N_NEAREST_NEIGHBORS as Float)
    }
    fn laplace(grid_view: GridView<Self>) -> Density {
        SECOND_ORDER_CENTRAL.iter()
            .flatten()
            .zip(stencil_coords(3, 3))
            .map(|(&weight, dpos)| (weight as Float) * grid_view.get_cell_at(dpos).density)
            .sum()
    }
}

impl Cell for ComplexDiffusionCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        let radius = if (grid_pos.x() > 50) & (grid_pos.y() > 50) {
            //randomize::f32_half_open_right(rng.next_u32()) * 1.0
            1.0
        } else {
            0.0
        };
        let radius = Float::sin(2.0*std::f64::consts::PI*(grid_pos.x() as Float)/100.0);
        let radius = radius * Float::sin(2.0*std::f64::consts::PI*(grid_pos.y() as Float)/100.0);
        //let theta = randomize::f32_half_open_right(rng.next_u32()) * 2.0 * PI;
        let theta = 0.0;
        Self{density: Density::from_polar(radius as Float, theta as Float)}
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        let new_density: Density = Self::laplace(grid_view)*Self::diffusion_constant()
            + self.density;
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