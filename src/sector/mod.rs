use crate::sector::solid::Solid;
use crate::sector::water::Water;
use crate::traits::{CountCellsOnSector, LightCoeff, MoveCoeff, ResourceCoeff};

pub mod solid;
pub mod water;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sector {
    Solid(Solid),
    Water(Water),
}

impl LightCoeff for Sector {
    fn get_light_coeff(&self) -> f32 {
        match self {
            Sector::Solid(solid) => solid.light_coefficient,
            Sector::Water(water) => water.light_coefficient,
        }
    }
}

impl MoveCoeff for Sector {
    fn get_move_coeff(&self) -> f32 {
        match self {
            Sector::Solid(solid) => solid.move_coefficient,
            Sector::Water(water) => water.move_coefficient,
        }
    }
}

impl ResourceCoeff for Sector {
    fn get_resource_coeff(&self) -> f32 {
        match self {
            Sector::Solid(solid) => solid.resource_coefficient,
            Sector::Water(water) => water.resource_coefficient,
        }
    }
}

impl CountCellsOnSector for Sector {
    fn get_count_cells(&self) -> usize {
        match self {
            Sector::Solid(solid) => solid.count_cells,
            Sector::Water(water) => water.count_cells,
        }
    }

    fn is_zero(&self) -> bool {
        self.get_count_cells() == 0
    }

    fn decrement_count_cells(&mut self) {
        match self {
            Sector::Solid(solid) => solid.count_cells -= 1,
            Sector::Water(water) => water.count_cells -= 1,
        }
    }

    fn increment_count_cells(&mut self) {
        match self {
            Sector::Solid(solid) => solid.count_cells += 1,
            Sector::Water(water) => water.count_cells += 1,
        }
    }
}
