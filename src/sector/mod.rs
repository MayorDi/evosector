use crate::sector::solid::Solid;
use crate::sector::water::Water;

pub mod solid;
pub mod water;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sector {
    Solid(Solid),
    Water(Water),
}