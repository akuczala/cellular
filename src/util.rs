pub type RandomGenerator = randomize::PCG32;
pub type Color = [u8; 4];

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