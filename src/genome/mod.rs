use crate::constants::COUNT_GENES;
use crate::genome::gene::Gene;
use crate::traits::Mutable;

pub mod gene;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Genome {
    pub genes: [Option<Gene>; COUNT_GENES],
}

impl Genome {
    pub fn new() -> Self {
        Self {
            genes: [None; COUNT_GENES],
        }
    }
}

impl Mutable for Genome {
    fn mutate(&mut self) -> bool {
        todo!()
    }
}

impl Default for Genome {
    fn default() -> Self {
        todo!()
    }
}
