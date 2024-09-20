use crate::cell::{Cell, Randomize};
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::grid::grid_view::GridView;
use crate::util::{gauss, map_to_unit_interval, modulo, RandomGenerator};
use palette::{Hsv, Pixel, Srgb};
use std::f32::consts::PI;

type Float = f32;
const J: Float = 1.0;
const H: Float = 0.0;
const DAMPING: Float = 0.0;
const TWO_PI: Float = 2.0 * PI;
const DT: Float = 0.01;
const TEMPERATURE: Float = 0.1;

#[derive(Clone, Default)]
pub struct XYModelCell {
    pub value: Float,
    pub velocity: Float,
}
impl XYModelCell {
    fn get_energy_from(&self, grid_view: &GridView<Self>, di: GridInt, dj: GridInt) -> Float {
        let other_cell = grid_view.get_cell_at_coord(di, dj);
        let self_angle = TWO_PI * self.value;
        let delta_angle = self_angle - TWO_PI * other_cell.value;
        -J * (delta_angle.cos())
    }
    pub fn get_energy(&self, grid_view: &GridView<Self>) -> Float {
        // this term is called 'kinetic' in processing, but it looks like an external field term
        let kinetic = -H * (TWO_PI * grid_view.get_cell_at_coord(0, 0).value).cos();
        let potential: Float = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(di, dj)| self.get_energy_from(grid_view, *di, *dj))
            .sum();
        return kinetic + potential;
    }
    fn get_force_from(&self, grid_view: &GridView<Self>, di: GridInt, dj: GridInt) -> Float {
        let other_cell = grid_view.get_cell_at_coord(di, dj);
        let self_angle = TWO_PI * self.value;
        let delta_angle = self_angle - TWO_PI * other_cell.value;
        -J * (delta_angle.sin()) + H * (self_angle.sin())
    }
    fn get_force(&self, grid_view: &GridView<Self>) -> Float {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(di, dj)| self.get_force_from(grid_view, *di, *dj))
            .sum()
    }
    fn create_gauss(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        let value = gauss(0.1, [10.0, 10.0], &target_pos, &grid_pos);
        self.value = modulo(self.value + value, 1.0);
    }
    fn create_defect(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        let (dx, dy) = (grid_pos.x - target_pos.x, grid_pos.y - target_pos.y);
        let (dx, dy) = (dx as Float, dy as Float);
        let scale: Float = 40.0;
        let _dist = (dx * dx + dy * dy) / scale.powi(2);
        let value = map_to_unit_interval(dy.atan2(dx), -PI, PI);
        self.value = modulo(self.value + value, 1.0);
    }
    fn thermal_update(&self, _grid_view: &GridView<Self>) -> Self {
        // need rng here
        todo!()
    }
}
impl Randomize for XYModelCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        let value = (grid_pos.x / 10) * (grid_pos.y / 10);
        let value = (value as Float) / 10.0;
        let _value = value + randomize::f32_half_open_right(rng.next_u32()) * 0.0;
        let value = randomize::f32_half_open_right(rng.next_u32()) * 1.0;
        Self {
            value,
            velocity: 0.0,
        }
    }
}
impl Cell for XYModelCell {
    

    fn update(&self, grid_view: GridView<Self>) -> Self {
        let velocity = self.velocity * (1.0 - DAMPING * DT) + DT * self.get_force(&grid_view);
        let value = modulo(self.value + DT * velocity, 1.0);
        Self { value, velocity }
    }

    fn draw(&self) -> [u8; 4] {
        let hue = self.value * 360.0;
        let rgb: [u8; 3] = Srgb::from(Hsv::new(hue, 1.0, 1.0)).into_format().into_raw();
        [rgb[0], rgb[1], rgb[2], 0]
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        self.create_defect(target_pos, grid_pos)
    }

    fn line_action(&mut self, _target_pos: &GridPos, _grid_pos: &GridPos, _alive: bool) {}
}

// impl Boundary<XYModelCell> {
//     fn free()
// }
