use crate::cell::Cell;
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;
use crate::util::{
    complex_to_hue, gauss, map_from_unit_interval, stencil_coords, RandomGenerator,
    SECOND_ORDER_CENTRAL,
};
use num_complex::Complex32;
use palette::{Hsv, LinSrgb, Pixel};
use std::f32::consts::PI;

type Float = f32;

const DT: Float = 0.1;

#[derive(Default, Clone)]
pub struct SchrodingerCell {
    real: Float,
    imag: Float,
    update_phase: CellDataLabel,
}
#[derive(Clone, Copy)]
enum CellDataLabel {
    Real,
    Imag,
}
impl Default for CellDataLabel {
    fn default() -> Self {
        Self::Real
    }
}

impl SchrodingerCell {
    fn laplace(grid_view: &GridView<Self>, label: CellDataLabel) -> Float {
        SECOND_ORDER_CENTRAL
            .iter()
            .flatten()
            .zip(stencil_coords(3, 3))
            .map(|(weight, dpos)| weight * grid_view.get_cell_at(dpos).get_data(label))
            .sum()
    }
    fn one_dimensional_hamiltonian(
        &self,
        grid_view: GridView<Self>,
        label: CellDataLabel,
    ) -> Float {
        let laplace: Float = [-1, 0, 1]
            .iter()
            .zip([1.0, -2.0, 1.0])
            .map(|(&di, weight)| {
                weight * grid_view.get_cell_at(GridPos::new(di, 0)).get_data(label)
            })
            .sum();
        -laplace
    }
    fn hamiltonian(&self, grid_view: GridView<Self>, label: CellDataLabel) -> Float {
        -Self::laplace(&grid_view, label)
            + Self::potential(&grid_view.origin) * self.get_data(label)
    }
    fn get_data(&self, label: CellDataLabel) -> Float {
        match label {
            CellDataLabel::Real => self.real,
            CellDataLabel::Imag => self.imag,
        }
    }
    fn potential(grid_pos: &GridPos) -> Float {
        Self::quartic_potential(grid_pos)
    }
    fn free_potential(grid_pos: &GridPos) -> Float {
        0.0
    }
    fn step_potential(grid_pos: &GridPos) -> Float {
        let (x, y) = (grid_pos.x() as Float, grid_pos.y() as Float);
        match x {
            x if x < 100.0 => 0.0,
            _ => 1.8,
        }
    }
    fn harmonic_potential(grid_pos: &GridPos) -> Float {
        let radius = 100.0;
        let (x, y) = (grid_pos.x() as Float, grid_pos.y() as Float);
        let x = x - radius;
        let y = y - radius;
        (x * x + y * y) / (Float::powi(radius, 2)) * 4.0
    }
    fn coupled_harmonic_potential(grid_pos: &GridPos) -> Float {
        let radius = 100.0;
        let (x, y) = (grid_pos.x() as Float, grid_pos.y() as Float);
        let x = x - radius;
        let y = y - radius;
        (x * x + y * y + ((x + y).powi(2))) / (Float::powi(radius, 2)) * 2.0
    }
    fn quartic_potential(grid_pos: &GridPos) -> Float {
        let radius = 100.0;
        let (x, y) = (grid_pos.x() as Float, grid_pos.y() as Float);
        let (x, y) = ((x - radius) / radius, (y - radius) / radius);
        let r_sq = (x * x + y * y);
        (-r_sq + 2.0 * r_sq * r_sq) * 1.0
    }
    fn circular_well(grid_pos: &GridPos) -> Float {
        let radius = 100.0;
        let (x, y) = (grid_pos.x() as Float, grid_pos.y() as Float);
        let (x, y) = ((x - radius) / radius, (y - radius) / radius);
        let r_sq = (x * x + y * y);
        match r_sq {
            r_sq if r_sq > 0.5 => 4.0,
            _ => 0.0,
        }
    }
}
impl Cell for SchrodingerCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        Self {
            real: 0.1
                * map_from_unit_interval(randomize::f32_half_open_right(rng.next_u32()), -1.0, 1.0),
            imag: 0.1
                * map_from_unit_interval(randomize::f32_half_open_right(rng.next_u32()), -1.0, 1.0),
            update_phase: CellDataLabel::Real,
        }
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        match self.update_phase {
            CellDataLabel::Real => {
                let real = self.real + DT * self.hamiltonian(grid_view, CellDataLabel::Imag);
                let imag = self.imag;
                let update_phase = CellDataLabel::Imag;
                Self {
                    real,
                    imag,
                    update_phase,
                }
            }
            CellDataLabel::Imag => {
                let real = self.real;
                let imag = self.imag - DT * self.hamiltonian(grid_view, CellDataLabel::Real);
                let update_phase = CellDataLabel::Real;
                Self {
                    real,
                    imag,
                    update_phase,
                }
            }
        }
    }

    fn draw(&self) -> [u8; 4] {
        //note: to display conserved probability you need to track and mix more timestamps
        let z = Complex32::new(self.real, self.imag);
        let hue = complex_to_hue(z);
        let value = z.norm();
        let rgb: [u8; 3] = LinSrgb::from(Hsv::new(hue, 1.0, value))
            .into_format()
            .into_raw();
        [rgb[0], rgb[1], rgb[2], 0]
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        let sigma = 10.0;
        let wavelength = 20.0;
        let wave_vec = [1.0, 0.0];
        let amplitude = 20.0 / (PI.sqrt() * sigma);
        let gauss_value = gauss(amplitude, [sigma, sigma], &target_pos, &grid_pos);
        let phase = 2.0
            * PI
            * ((grid_pos.x() as Float) * wave_vec[0] + (grid_pos.y() as Float) * wave_vec[1])
            / wavelength;
        let phase = phase * 0.0;
        self.real += gauss_value * phase.cos();
        self.imag += gauss_value * phase.sin();
    }

    fn line_action(&mut self, target_pos: &GridPos, grid_pos: &GridPos, alive: bool) {}
}
