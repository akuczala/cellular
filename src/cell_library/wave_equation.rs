use crate::cell::{Cell, HasColor, Randomize};
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;
use crate::util::{
    gauss, map_to_unit_interval, stencil_coords, RandomGenerator, SECOND_ORDER_CENTRAL,
};
use std::f32::consts::PI;
type Float = f32;

const DT: Float = 0.01;
const DAMPING: Float = 0.01;
const MASS: Float = 0.0;

#[derive(Clone, Debug, Default)]
pub struct WaveCell {
    value: Float,
    velocity: Float,
}
impl WaveCell {
    fn laplace(grid_view: GridView<Self>) -> Float {
        SECOND_ORDER_CENTRAL
            .iter()
            .flatten()
            .zip(stencil_coords(3, 3))
            .map(|(weight, dpos)| weight * grid_view.get_cell_at(dpos).value)
            .sum()
    }
    fn speed(grid_pos: &GridPos) -> Float {
        Self::single_slit_speed(grid_pos)
    }
    fn constant_speed(_grid_pos: &GridPos) -> Float {
        1.0
    }
    fn material_interface_speed(grid_pos: &GridPos) -> Float {
        match grid_pos.x {
            y if y < 100 => 1.0,
            _y => 0.0,
        }
    }
    fn single_slit_speed(grid_pos: &GridPos) -> Float {
        let (x, y) = (grid_pos.x as Float, grid_pos.y as Float);
        match (x, y) {
            (x, y) if (x > 95.0) & (x < 105.0) & ((y < 80.0) | (y > 120.0)) => 0.0,
            _ => 1.0,
        }
    }
    fn mode(nx: i32, ny: i32, grid_pos: GridPos) -> Self {
        let (nx, ny) = (nx as Float, ny as Float);
        let value = Float::sin(2.0 * nx * PI * (grid_pos.x as Float) / 200.0);
        let value = value * Float::sin(2.0 * ny * PI * (grid_pos.y as Float) / 200.0);
        Self {
            value,
            velocity: 0.0,
        }
    }
}
impl Randomize for WaveCell {
    fn random(rng: &mut RandomGenerator, _grid_pos: GridPos) -> Self {
        let value = randomize::f32_half_open_right(rng.next_u32()) * 2.0 - 1.0;
        Self {
            value,
            velocity: 0.0,
        }
        //Self::mode(5, 3, grid_pos)
    }
}
impl HasColor for WaveCell {
    fn draw(&self) -> [u8; 4] {
        let frac = map_to_unit_interval(self.value, 0.0, 1.0).clamp(-1.0, 1.0);
        let pos = (frac.clamp(0.0, 1.0) * (0xff as Float)) as u8;
        let neg = (-1.0 * frac.clamp(-1.0, 0.0) * (0xff as Float)) as u8;
        [pos, neg, neg, 0]
    }
}
impl Cell for WaveCell {
    fn update(&self, grid_view: GridView<Self>) -> Self {
        let velocity = self.velocity * (1.0 - DAMPING * DT)
            + Self::speed(&grid_view.origin) * Self::laplace(grid_view) * DT
            - self.value * MASS * DT;
        let value = self.value + velocity * DT;
        Self { value, velocity }
    }

    fn toggle(&mut self, _target_pos: &GridPos, _grid_pos: &GridPos) {}

    fn line_action(&mut self, target_pos: &GridPos, grid_pos: &GridPos, _alive: bool) {
        let sigma_x: Float = 20.0;
        let sigma_y: Float = 50.0;
        let wavelength = 5.0;
        let wave_vec = [1.0, 0.0];
        let amplitude = 10.0 / (PI * (sigma_x.powi(2) + sigma_y.powi(2))).sqrt();
        let gauss_value = gauss(amplitude, [sigma_x, sigma_y], &target_pos, &grid_pos);
        let phase =
            2.0 * PI * ((grid_pos.x as Float) * wave_vec[0] + (grid_pos.y as Float) * wave_vec[1])
                / wavelength;
        self.value += gauss_value * phase.cos();
        self.velocity += 1.0 * gauss_value * phase.sin();
    }
}
