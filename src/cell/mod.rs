use crate::constants::{DEFAULT_ENERGY_CELL, DEFAULT_PROTECTION_BODY_CELL};
use crate::genome::Genome;
use nalgebra::Vector2;

#[derive(Debug, Clone)]
pub struct Cell {
    energy: f32,
    position: Vector2<f32>,
    genome: Genome,
    protection_body: f32,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            energy: DEFAULT_ENERGY_CELL,
            position,
            genome: Genome::new(),
            protection_body: DEFAULT_PROTECTION_BODY_CELL,
        }
    }
}
