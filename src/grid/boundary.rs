use crate::cell::Cell;

#[derive(Clone,Debug)]
pub enum Boundary<C: Cell> {
    Periodic,
    Constant(C)
}
impl<C: Cell> Boundary<C> {
    pub fn empty() -> Self {
        Self::Constant(C::default())
    }
}