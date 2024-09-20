use crate::cell::{Cell, HasColor, Randomize};
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;
use crate::util::NEAREST_NEIGHBORS;
use crate::util::{Color, RandomGenerator};

const BIRTH_RULE: [bool; 9] = [false, false, false, true, false, false, false, false, false];
const SURVIVE_RULE: [bool; 9] = [false, false, true, true, false, false, false, false, false];

const INITIAL_FILL: f32 = 0.3;

#[derive(Clone, Copy, Debug, Default)]
pub struct ConwayCell {
    pub alive: bool,
    // Used for the trail effect. Always 255 if `self.alive` is true (We could
    // use an enum for Cell, but it makes several functions slightly more
    // complex, and doesn't actually make anything any simpler here, or save any
    // memory, so we don't)
    pub heat: u8,
}

impl ConwayCell {
    pub fn new(alive: bool) -> Self {
        Self { alive, heat: 0 }
    }
    fn count_neibs(grid_view: GridView<Self>) -> usize {
        NEAREST_NEIGHBORS
            .iter()
            .map(|dxy| grid_view.get_cell_at_coord(dxy[0], dxy[1]).alive as usize)
            .sum()
    }
    fn set_alive(&mut self, alive: bool) {
        *self = self.next_state(alive);
    }
    #[must_use]
    fn next_state(mut self, alive: bool) -> Self {
        self.alive = alive;
        if self.alive {
            self.heat = 255;
        } else {
            self.heat = self.heat.saturating_sub(1);
        }
        self
    }
}
impl Randomize for ConwayCell {
    fn random(rng: &mut RandomGenerator, _grid_pos: GridPos) -> Self {
        let alive = randomize::f32_half_open_right(rng.next_u32()) > INITIAL_FILL;
        ConwayCell::new(alive)
    }
}
impl HasColor for ConwayCell {
    fn draw(&self) -> Color {
        if self.alive {
            [0, 0xff, 0xff, 0xff]
        } else {
            [0, 0, self.heat, 0xff]
        }
    }
}
impl Cell for ConwayCell {
    fn update(&self, grid_view: GridView<Self>) -> Self {
        let n = ConwayCell::count_neibs(grid_view);
        let next_alive = if self.alive {
            SURVIVE_RULE[n]
        } else {
            BIRTH_RULE[n]
        };
        self.next_state(next_alive)
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        let was_alive = self.alive;
        if (target_pos.x == grid_pos.x) & (target_pos.y == grid_pos.y) {
            self.set_alive(!was_alive);
        }
    }

    fn line_action(&mut self, _target_pos: &GridPos, _grid_pos: &GridPos, alive: bool) {
        self.set_alive(alive);
    }
}
