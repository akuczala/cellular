use crate::cell::{Cell, HasColor, Randomize};
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;
use crate::util::{
    complex_to_hue, gauss, stencil_coords, Color, RandomGenerator, NEAREST_NEIGHBORS,
    N_NEAREST_NEIGHBORS, SECOND_ORDER_CENTRAL,
};
use num_complex::Complex64;
use palette::{Hsv, LinSrgb, Pixel};

type Density = Complex64;
type Float = f64;
const MAX_ABS: Float = 1.0;

#[derive(Default, Clone)]
pub struct ComplexDiffusionCell {
    pub(crate) density: Density,
}
impl ComplexDiffusionCell {
    fn diffusion_constant() -> Density {
        0.005 * Density::i()
    }
    fn avg_neighbors(grid_view: GridView<Self>) -> Density {
        NEAREST_NEIGHBORS
            .iter()
            .map(|dxy| grid_view.get_cell_at_coord(dxy[0], dxy[1]).density)
            .sum::<Density>()
            / (N_NEAREST_NEIGHBORS as Float)
    }
    fn laplace(grid_view: GridView<Self>) -> Density {
        SECOND_ORDER_CENTRAL
            .iter()
            .flatten()
            .zip(stencil_coords(3, 3))
            .map(|(&weight, dpos)| (weight as Float) * grid_view.get_cell_at(dpos).density)
            .sum()
    }
}
impl Randomize for ComplexDiffusionCell {
    fn random(_rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        let _radius = if (grid_pos.x > 50) & (grid_pos.y > 50) {
            //randomize::f32_half_open_right(rng.next_u32()) * 1.0
            1.0
        } else {
            0.0
        };
        let radius = Float::sin(2.0 * std::f64::consts::PI * (grid_pos.x as Float) / 100.0);
        let radius =
            radius * Float::sin(2.0 * std::f64::consts::PI * (grid_pos.y as Float) / 100.0);
        //let theta = randomize::f32_half_open_right(rng.next_u32()) * 2.0 * PI;
        let theta = 0.0;
        Self {
            density: Density::from_polar(radius as Float, theta as Float),
        }
    }
}
impl HasColor for ComplexDiffusionCell {
    fn draw(&self) -> Color {
        let hue = complex_to_hue(self.density);
        let value = self.density.norm();
        let rgb: [u8; 3] = LinSrgb::from(Hsv::new(hue, 1.0, value))
            .into_format()
            .into_raw();
        [rgb[0], rgb[1], rgb[2], 0]
    }
}
impl Cell for ComplexDiffusionCell {
    fn update(&self, grid_view: GridView<Self>) -> Self {
        let new_density: Density =
            Self::laplace(grid_view) * Self::diffusion_constant() + self.density;
        Self {
            density: new_density,
        }
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        self.density +=
            gauss(1.0, [20.0, 20.0], &target_pos, &grid_pos) * Density::new(MAX_ABS, 0.0);
    }

    fn line_action(&mut self, _target_pos: &GridPos, _grid_pos: &GridPos, _alive: bool) {}
}

#[test]
fn test_draw() {
    
    let cell = ComplexDiffusionCell {
        density: Density::new(-1.0, 0.0),
    };
    let hue = complex_to_hue(cell.density);
    let value = cell.density.norm();
    //assert_eq!(hue, 0.5);
    assert_eq!(value, 1.0);
    let hsv = Hsv::new(hue, 1.0, value);
    println!("{:?}", hsv);
    //println!("{:?}", Rgb::from(hsv));
    assert_eq!(cell.draw(), [0, 0xff, 0xff, 0]);

    let cell = ComplexDiffusionCell {
        density: Density::new(0.0, 1.0),
    };
    let hue = complex_to_hue(cell.density);
    let value = cell.density.norm();
    assert_eq!(hue, 0.75);
    assert_eq!(value, 1.0);
    //println!("{:?}", Rgb::from(Hsv::new(hue, 1.0, value)));
    assert_eq!(cell.draw(), [0, 0xff, 0xff, 0]);
}
