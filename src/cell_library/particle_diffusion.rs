use crate::cell::Cell;
use crate::grid::grid_pos::GridPos;
use crate::grid::grid_view::GridView;
use crate::util::{RandomGenerator, map_to_unit_interval, generate_seed, gauss};

type ParticleCount = u32;
#[derive(Clone,Default)]
struct ParticleCounter {
    pub up: ParticleCount,
    pub down: ParticleCount,
    pub left: ParticleCount,
    pub right: ParticleCount
}

impl ParticleCounter {
    pub fn randomize_n(n: ParticleCount, rng: &mut RandomGenerator) -> ParticleCounter {
        let mut counts: [ParticleCount; 4] = [0, 0, 0, 0];
        let mut random_slot = randomize::RandRangeU32::new(0,3);
        for _ in 0..n {
            counts[random_slot.sample(rng) as usize] += 1;
        }
        Self::from_arr(&counts)
    }
    pub fn from_arr(arr: &[ParticleCount; 4]) -> Self {
        Self{up: arr[0], down: arr[1], left: arr[2], right: arr[3]}
    }
    pub fn to_arr(&self) -> [ParticleCount; 4] {
        [self.up, self.down, self.left, self.right]
    }
    pub fn total(&self) -> ParticleCount {
        self.to_arr().iter().sum()
    }
}

#[derive(Clone)]
pub struct ParticleDiffusionCell {
    particles: ParticleCounter,
    rng: RandomGenerator
}
impl Default for ParticleDiffusionCell {
    fn default() -> Self {
        let mut rng: randomize::PCG32 = generate_seed().into();
        Self{particles: ParticleCounter::default(), rng}
    }
}
impl ParticleDiffusionCell {
    fn get_n_incoming(grid_view: GridView<Self>) -> ParticleCount {
        grid_view.get_cell_at_coord(-1,0).particles.right +
            grid_view.get_cell_at_coord(1,0).particles.left +
            grid_view.get_cell_at_coord(0,1).particles.up +
            grid_view.get_cell_at_coord(0,-1).particles.down
    }
}
impl Cell for ParticleDiffusionCell {
    fn random(rng: &mut RandomGenerator, grid_pos: GridPos) -> Self {
        let mut rng: randomize::PCG32 = generate_seed().into();
        let n = randomize::RandRangeU32::new(0,5).sample(&mut rng);
        let particles = ParticleCounter::randomize_n(n, &mut rng);
        Self{particles, rng}
    }

    fn update(&self, grid_view: GridView<Self>) -> Self {
        let n = Self::get_n_incoming(grid_view);
        let mut rng = self.rng.clone();
        let particles = ParticleCounter::randomize_n(n, &mut rng);
        Self{ particles, rng}
    }

    fn draw(&self) -> [u8; 4] {
        let frac = map_to_unit_interval(
            self.particles.total() as f32, 0.0, 10.0
        ).clamp(0.0, 1.0);
        let shade = (frac * (0xff as f32)) as u8;
        [shade, shade, shade, 0]
    }

    fn toggle(&mut self, target_pos: &GridPos, grid_pos: &GridPos) {

    }

    fn line_action(&mut self, target_pos: &GridPos, grid_pos: &GridPos, alive: bool) {
        let mut rng: randomize::PCG32 = generate_seed().into();
        let gauss_value = gauss(10.0, 20.0, &target_pos, &grid_pos);
        self.particles = ParticleCounter::randomize_n(gauss_value as ParticleCount, &mut rng);
    }
}