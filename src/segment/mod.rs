use crate::segment::solid::Solid;
use crate::segment::water::Water;

pub mod solid;
pub mod water;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Segment {
    Solid(Solid),
    Water(Water)
}
