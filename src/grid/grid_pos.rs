pub type GridInt = i32;

#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct GridPos {
    pub x: GridInt,
    pub y: GridInt,
}
impl GridPos {
    pub fn new(x: GridInt, y: GridInt) -> Self {
        GridPos { x, y }
    }
}
