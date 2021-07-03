use crate::grid::grid_pos::{GridInt, GridPos};
use std::ops::Range;
use std::f32::consts::PI;
use num_complex::Complex32;

pub const NEAREST_NEIGHBORS: [[i32; 2]; 8] = [[1,0],[1,1],[0,1],[-1,1],[-1,0],[-1,-1],[0,-1],[1,-1]];
pub const N_NEAREST_NEIGHBORS: u8 = 8;

pub const SECOND_ORDER_CENTRAL: [[f32; 3]; 3] = [
    [0.0,  1.0, 0.0],
    [1.0, -4.0, 1.0],
    [0.0,  1.0, 0.0]
];

pub type RandomGenerator = randomize::PCG32;
pub type Color = [u8; 4];

//width and height must be odd
pub fn stencil_coords(width: GridInt, height: GridInt) -> impl Iterator<Item=GridPos> {
    let get_range: fn(GridInt) -> Range<GridInt> = |n| (-n / 2.. n / 2 + 1);
    get_range(height).map(move |dy| get_range(width)
        .map(move |dx| GridPos::new(dx, dy))
    ).flatten()
}
pub fn modulo(lhs: i32, rhs: i32) -> i32 {
    let r = lhs % rhs;
    if r < 0 {
        return if rhs > 0 { r + rhs } else { r - rhs }
    }
    r
}

/// Generate a pseudorandom seed for the game's PRNG.
pub fn generate_seed() -> (u64, u64) {
    use byteorder::{ByteOrder, NativeEndian};
    use getrandom::getrandom;

    let mut seed = [0_u8; 16];

    getrandom(&mut seed).expect("failed to getrandom");

    (
        NativeEndian::read_u64(&seed[0..8]),
        NativeEndian::read_u64(&seed[8..16]),
    )
}

pub fn complex_to_hue<I: std::convert::Into<Complex32>>(z: I) -> f32 {
    let z = z.into();
    (z.arg() / PI + 1.0) / 2.0 * 360.0
}

pub fn map_to_unit_interval<I>(x: I, min: I, max: I) -> I
where I: std::ops::Sub<I, Output=I> + std::ops::Div<I, Output=I> + Copy {
    (x - min)/(max - min)
}