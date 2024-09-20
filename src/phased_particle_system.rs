use crate::cell::{Cell, HasColor, System};
use crate::grid::grid_pos::{GridInt, GridPos};
use crate::grid::grid_view::GridView;
use crate::grid::Grid;
use crate::util::{
    complex_to_hue, generate_seed, map_to_unit_interval, modulo, Color, RandomGenerator,
};
use num_complex::Complex32;
use palette::{Hsv, LinSrgb, Pixel};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}
impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Default)]
struct Particle {
    pub phase: Complex32,
    pub pos: GridPos,
    pub last_direction: Direction,
}
impl Particle {
    fn update(&self, rng: &mut RandomGenerator, grid: &Grid<PhasedParticleCell>) -> Self {
        let random_int = randomize::RandRangeU32::new(0, 1);
        let dx = (random_int.sample(rng) as GridInt) * 2 - 1;
        let direction = match dx {
            -1 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("Should never get here"),
        };
        let pos = GridPos::new(modulo(self.pos.x + dx, grid.width as GridInt), self.pos.y);
        let phase = match direction {
            x if x == self.last_direction => self.phase,
            _ => self.phase * Complex32::i(),
        };
        Self {
            phase,
            pos,
            last_direction: direction,
        }
    }
    fn old_update(&self, rng: &mut RandomGenerator, grid: &Grid<PhasedParticleCell>) -> Self {
        let random_int = randomize::RandRangeU32::new(0, 2);
        let _pos = GridPos::new(
            modulo(
                self.pos.x + random_int.sample(rng) as GridInt - 1,
                grid.width as GridInt,
            ),
            modulo(
                self.pos.y + random_int.sample(rng) as GridInt - 1,
                grid.height as GridInt,
            ),
        );
        let _phase = self.phase * Complex32::i();
        unimplemented!()
        //Self{phase, pos}
    }
}
pub struct PhasedParticleSystem {
    particles: Vec<Particle>,
    pub grid: Grid<PhasedParticleCell>,
    rng: RandomGenerator,
}
impl PhasedParticleSystem {
    pub fn new(n: usize, grid: Grid<PhasedParticleCell>) -> Self {
        let mut rng: randomize::PCG32 = generate_seed().into();
        let x_center = (grid.width / 2) as u32;
        let y_center = (grid.height / 2) as u32;
        let random_int_x = randomize::RandRangeU32::new(x_center, x_center);
        let random_int_y = randomize::RandRangeU32::new(y_center - 2, y_center + 2);
        let particles = (0..n)
            .map(|_| Particle {
                phase: Complex32::new(1.0, 0.0),
                pos: GridPos::new(
                    random_int_x.sample(&mut rng) as GridInt,
                    random_int_y.sample(&mut rng) as GridInt,
                ),
                last_direction: match randomize::RandRangeU32::new(0, 1).sample(&mut rng) {
                    0 => Direction::Left,
                    1 => Direction::Right,
                    _ => panic!(),
                },
            })
            .collect();
        Self {
            particles,
            grid,
            rng,
        }
    }
}

impl System<PhasedParticleCell> for PhasedParticleSystem {
    fn update(&mut self) {
        //update particles
        for particle in &mut self.particles {
            *particle = particle.update(&mut self.rng, &self.grid);
        }
        // loop through particles, buildings mut list of cells to write to scratch
        // this approach is unnecessarily slow
        let mut cells: HashMap<GridPos, PhasedParticleCell> = HashMap::new();
        for particle in &self.particles {
            let pos = particle.pos;
            let new_cell = match cells.get(&pos) {
                None => PhasedParticleCell {
                    phase: particle.phase,
                },
                Some(cell) => PhasedParticleCell {
                    phase: cell.phase + particle.phase,
                },
            };
            cells.insert(pos, new_cell);
        }
        //clear all cells
        for grid_pos in self.grid.get_grid_pos_iter() {
            self.grid
                .set_scatch_cell_at(grid_pos, PhasedParticleCell::default());
        }
        for (pos, cell) in cells.drain() {
            let cell = PhasedParticleCell {
                phase: cell.phase / (self.particles.len() as f32 / 1000.0),
            };
            self.grid.set_scatch_cell_at(pos, cell)
        }

        self.grid.swap()
    }

    fn update_cell(
        &self,
        _grid_view: GridView<PhasedParticleCell>,
        _cell: &PhasedParticleCell,
    ) -> PhasedParticleCell {
        todo!()
    }

    fn toggle(&mut self, x: isize, y: isize) -> bool {
        self.particles.push(Particle {
            phase: Complex32::new(1.0, 0.0),
            pos: GridPos::new(x as GridInt, y as GridInt),
            last_direction: Default::default(),
        });
        // println!("---");
        // for particle in &self.particles {
        //     println!("{:?}, {:?}",particle.phase,particle.pos);
        // }
        true
    }

    fn line_action(&mut self, _target_pos: GridPos, _alive: bool) {}
}

#[derive(Default, Clone)]
pub struct PhasedParticleCell {
    pub phase: Complex32,
}
impl PhasedParticleCell {
    fn draw_complex(&self) -> Color {
        let hue = complex_to_hue(self.phase);
        let value = (self.phase.norm() * 10.0).clamp(0.0, 1.0);
        let rgb: [u8; 3] = LinSrgb::from(Hsv::new(hue, 1.0, value))
            .into_format()
            .into_raw();
        [rgb[0], rgb[1], rgb[2], 0]
    }
    fn draw_real(&self) -> Color {
        let frac = map_to_unit_interval(self.phase.re / 1.0, 0.0, 1.0).clamp(-1.0, 1.0);
        let pos = (frac.clamp(0.0, 1.0) * (0xff as f32)) as u8;
        let neg = (-1.0 * frac.clamp(-1.0, 0.0) * (0xff as f32)) as u8;
        [pos, neg, neg, 0]
    }
}
impl HasColor for PhasedParticleCell {
    fn draw(&self) -> Color {
        self.draw_complex()
    }
}
impl Cell for PhasedParticleCell {
    fn update(&self, _grid_view: GridView<Self>) -> Self {
        unimplemented!()
    }

    fn toggle(&mut self, _target_pos: &GridPos, _grid_pos: &GridPos) {
        todo!()
    }

    fn line_action(&mut self, _target_pos: &GridPos, _grid_pos: &GridPos, _alive: bool) {
        todo!()
    }
}
