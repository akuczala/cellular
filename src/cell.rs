use crate::grid::grid_view::GridView;

const BIRTH_RULE: [bool; 9] = [false, false, false, true, false, false, false, false, false];
const SURVIVE_RULE: [bool; 9] = [false, false, true, true, false, false, false, false, false];
const NEIGHBOR_OFFSETS: [[i32; 2]; 8] = [[1,0],[1,1],[0,1],[-1,1],[-1,0],[-1,-1],[0,-1],[1,-1]];

#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub alive: bool,
    // Used for the trail effect. Always 255 if `self.alive` is true (We could
    // use an enum for Cell, but it makes several functions slightly more
    // complex, and doesn't actually make anything any simpler here, or save any
    // memory, so we don't)
    pub heat: u8,
}

impl Cell {
    pub fn new(alive: bool) -> Self {
        Self { alive, heat: 0 }
    }

    #[must_use]
    pub fn update(self, grid_view: GridView) -> Self {
        let n = Cell::count_neibs(grid_view);
        let next_alive = if self.alive {
            SURVIVE_RULE[n]
        } else {
            BIRTH_RULE[n]
        };
        self.next_state(next_alive)
    }
    fn count_neibs(grid_view: GridView) -> usize {
        NEIGHBOR_OFFSETS
            .iter()
            .map(|dxy| { grid_view.get_cell_at(dxy[0], dxy[1]).alive as usize })
            .sum()
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

    pub fn set_alive(&mut self, alive: bool) {
        *self = self.next_state(alive);
    }

    pub fn cool_off(&mut self, decay: f32) {
        if !self.alive {
            let heat = (self.heat as f32 * decay).min(255.0).max(0.0);
            assert!(heat.is_finite());
            self.heat = heat as u8;
        }
    }
}