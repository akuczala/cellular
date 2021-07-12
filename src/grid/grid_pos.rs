pub type GridInt = i32;

#[derive(Copy,Clone)]
pub struct GridPos {
    x: GridInt,
    y: GridInt
}
impl GridPos {
    pub fn new(x: GridInt, y: GridInt) -> Self {
        GridPos{x, y}
    }
    pub fn x(&self) -> GridInt {
        self.x
    }
    pub fn y(&self) -> GridInt {
        self.y
    }
}