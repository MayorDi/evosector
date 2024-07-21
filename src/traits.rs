use crate::grid::Grid;

pub trait Render {
    fn render(&self);
}

pub trait Behavior {
    type GlobalImpact;
    fn update(&mut self, grid: &mut Grid) -> Option<Self::GlobalImpact>;
}

pub trait Mutable {
    fn mutate(&mut self) -> bool;
}
