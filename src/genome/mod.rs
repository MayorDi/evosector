use crate::constants::COUNT_GENES;
use crate::genome::gene::Gene;

pub mod gene;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Genome {
    genes: [Option<Gene>; COUNT_GENES],
    step: usize,
    protection: f32,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            genes: [None; COUNT_GENES],
            step: 0,
            protection: 1.0,
        }
    }
}
