pub(crate) fn modulo(lhs: i32, rhs: i32) -> i32 {
    let r = lhs % rhs;
    if r < 0 {
        return if rhs > 0 { r + rhs } else { r - rhs }
    }
    r
}
