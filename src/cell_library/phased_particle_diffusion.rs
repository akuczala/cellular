use num_complex::Complex32;
use crate::util::RandomGenerator;
use crate::cell::Cell;
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;

#[derive(Default)]
struct Particles {
    n: i32,
    phases: [Vec<Complex32>; 4]
}
impl Particles {
    pub fn distribute(&mut self, phases: Vec<Complex32>, rng: &mut RandomGenerator) {
        let mut random_slot = randomize::RandRangeU32::new(0,3);
        for phase in phases.iter() {
            let slot = random_slot.sample(rng) as usize;
            self.phases[slot].push(*phase);
        }
    }
}
#[derive(Default)]
pub struct PhasedParticleDiffusionCell {
    particles: Particles,
    rng: RandomGenerator
}
impl PhasedParticleDiffusionCell {
    fn distribute_incoming(&mut self, grid_view: GridView<Self>) -> Particles {
        let mut particles = Particles::default();
        particles.distribute(grid_view.get_cell_at_coord(-1,0).particles.right, &mut self.rng);
        particles.distribute(grid_view.get_cell_at_coord(1,0).particles.left, &mut self.rng);
        particles.distribute(grid_view.get_cell_at_coord(0,1).particles.up, &mut self.rng);
        particles.distribute(grid_view.get_cell_at_coord(0,-1).particles.down, &mut self.rng);
        particles
    }

}
impl Cell for PhasedParticleDiffusionCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        todo!()
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        todo!()
    }

    fn draw(&self) -> [u8; 4] {
        todo!()
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        todo!()
    }

    fn line_action(&mut self, grid_pos: GridPos, alive: bool) {
        todo!()
    }
}