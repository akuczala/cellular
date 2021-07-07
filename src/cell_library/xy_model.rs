use crate::grid::grid_view::GridView;
use std::f32::consts::PI;
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::cell::Cell;
use crate::util::{RandomGenerator, modulo};
use palette::{Hsv, Srgb, Pixel};

type Float = f32;
const J: Float = 1.0;
const H: Float = 0.0;
const TWO_PI: Float = 2.0*PI;
const DT: Float = 0.005;


#[derive(Clone,Default)]
pub struct XYModelCell {
    pub value: Float,
    pub velocity: Float
}
impl XYModelCell {
    fn get_force_from(&self, grid_view: &GridView<Self>, di: GridInt, dj: GridInt) -> Float {
        let other_cell = grid_view.get_cell_at_coord(di, dj);
        let self_angle = TWO_PI * self.value;
        let delta_angle = self_angle - TWO_PI * other_cell.value;
        -J * (delta_angle.sin()) + H*(self_angle.sin())
    }
    fn get_force(&self, grid_view: &GridView<Self>) -> Float {
        [(1,0),(-1,0),(0,1),(0,-1)].iter()
            .map(|(di,dj)| self.get_force_from(grid_view, *di, *dj))
            .sum()
    }
}
impl Cell for XYModelCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        Self{
            value: randomize::f32_half_open_right(rng.next_u32()) * 1.0,
            velocity: 0.0
        }
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        let velocity = self.velocity + DT * self.get_force(&grid_view);
        let value = modulo(self.value + DT * velocity, 1.0);
        Self{value, velocity}
    }

    fn draw(&self) -> [u8; 4] {
        let hue = self.value * 360.0;
        let rgb: [u8; 3] = Srgb::from(Hsv::new(hue, 1.0, 1.0)).into_format().into_raw();
        [rgb[0], rgb[1], rgb[2], 0]
    }

    fn toggle(&mut self) {

    }

    fn line_action(&mut self, alive: bool) {
        self.velocity = 0.5;
    }
}

// impl Boundary<XYModelCell> {
//     fn free()
// }