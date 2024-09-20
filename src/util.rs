use crate::grid::grid_pos::{GridInt, GridPos};

use num_complex::{Complex};
use num_traits::{Float, Num, NumCast};

use std::ops::Range;

pub const NEAREST_NEIGHBORS: [[i32; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];
pub const N_NEAREST_NEIGHBORS: u8 = 8;

pub const SECOND_ORDER_CENTRAL: [[f32; 3]; 3] =
    [[0.0, 1.0, 0.0], [1.0, -4.0, 1.0], [0.0, 1.0, 0.0]];
// pub const SECOND_ORDER_CENTRAL_5: [[f32; 5]; 5] = [
//     [-1.0/12.0, 4.0/3.0, -5.0/2.0, 4.0/3.0, -1.0/12.0]
// ]
pub const SECOND_ORDER_CENTRAL_CROSS_SECTION: [[f32; 3]; 3] =
    [[0.0, 1.0, 0.0], [1.0, -6.0, 1.0], [0.0, 1.0, 0.0]];

pub type RandomGenerator = randomize::PCG32;
pub type Color = [u8; 4];

//width and height must be odd
pub fn stencil_coords(width: GridInt, height: GridInt) -> impl Iterator<Item = GridPos> {
    let get_range: fn(GridInt) -> Range<GridInt> = |n| (-n / 2..n / 2 + 1);
    get_range(height)
        .map(move |dy| get_range(width).map(move |dx| GridPos::new(dx, dy)))
        .flatten()
}
pub fn modulo<I: Num + Copy + std::cmp::PartialOrd>(lhs: I, rhs: I) -> I {
    let r = lhs % rhs;
    if r < I::zero() {
        return if rhs > I::zero() { r + rhs } else { r - rhs };
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

pub fn complex_to_hue<T: Float>(z: Complex<T>) -> T {
    let pi = T::from(180).unwrap().to_radians();
    (z.arg() / pi + T::one()) * T::from(180).unwrap()
}

pub fn map_to_unit_interval<I>(x: I, min: I, max: I) -> I
where
    I: Num + Copy,
{
    (x - min) / (max - min)
}
pub fn map_from_unit_interval<I: Num + Copy>(x: I, min: I, max: I) -> I {
    min + x * (max - min)
}

pub fn gauss<I: Float>(amplitude: I, sigma: [I; 2], mean: &GridPos, grid_pos: &GridPos) -> I {
    let (x, y) = (
        I::from(grid_pos.x).unwrap(),
        I::from(grid_pos.y).unwrap(),
    );
    let (mean_x, mean_y) = (I::from(mean.x).unwrap(), I::from(mean.y).unwrap());
    let (distx, disty) = ((x - mean_x) / sigma[0], (y - mean_y) / sigma[1]);
    let gauss_arg = -(distx.powi(2) + disty.powi(2));
    amplitude * gauss_arg.exp()
}
