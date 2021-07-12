use num_complex::Complex32;
use crate::util::RandomGenerator;
use crate::cell::Cell;
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;
#[derive(Default)]
struct Particle {
    phase: Complex32,
    pos: GridPos
}
#[derive(Default)]
pub struct Particles {
    particles: Vec<Particle>
}
#[derive(Default)]
pub struct PhasedParticleDiffusionCell {
    particles: Particles,
    rng: RandomGenerator
}
impl PhasedParticleDiffusionCell {


}
impl Cell for PhasedParticleDiffusionCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        todo!()
    }

    fn update(&self, grid_view: GridView<'a, Self>) -> Self {
        todo!()
    }

    fn draw(&self) -> [u8; 4] {
        todo!()
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {
        todo!()
    }

    fn line_action(&mut self, target_pos: &GridPos, grid_pos: &GridPos, alive: bool) {
        todo!()
    }
}