use crate::cell::{Cell, HasColor, Randomize};
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;
use crate::util::{Color, RandomGenerator};
use randomize::RandRangeU32;

const CRITICAL_HEIGHT: i32 = 4;

#[derive(Debug, Default, Clone)]
pub struct AbelianSandpileCell {
    pub height: i32,
}
impl AbelianSandpileCell {
    fn get_neighbor_sand(grid_view: &GridView<Self>) -> i32 {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(di, dj)| grid_view.get_cell_at_coord(*di, *dj).will_topple() as i32)
            .sum()
    }
    fn will_topple(&self) -> bool {
        self.height >= CRITICAL_HEIGHT
    }
    fn colormap_1(&self) -> Color {
        match self.height {
            0 => [0, 0, 0, 0],
            1 => [0xff, 0, 0xff, 0],
            2 => [0, 0, 0xff, 0],
            3 => [0, 0xff, 0, 0],
            4 => [0xff, 0xff, 0, 0],
            _ => [0xff, 0, 0, 0],
        }
    }
    fn grayscale(&self) -> Color {
        let shade = (self.height * 25).clamp(0, 0xff) as u8;
        [shade, shade / 2, shade / 4, 0]
    }
}
impl Randomize for AbelianSandpileCell {
    fn random(rng: &mut RandomGenerator, _grid_pos: GridPos) -> Self {
        let rand_int = RandRangeU32::new(0, 4);
        Self {
            height: rand_int.sample(rng) as i32,
        }
    }
}
impl HasColor for AbelianSandpileCell {
    fn draw(&self) -> Color {
        self.colormap_1()
    }
}
impl Cell for AbelianSandpileCell {
    fn update(&self, grid_view: GridView<Self>) -> Self {
        let sand_in = Self::get_neighbor_sand(&grid_view);
        let height = match self.will_topple() {
            true => self.height - CRITICAL_HEIGHT + sand_in,
            false => self.height + sand_in,
        };
        Self { height }
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        if (grid_pos.x == target_pos.x) & (grid_pos.y == target_pos.y) {
            self.height = 10_i32.pow(5) * 2;
        }
    }

    fn line_action(&mut self, _target_pos: &GridPos, _grid_pos: &GridPos, _alive: bool) {}
}
