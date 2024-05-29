use crate::constants::COUNT_GENES;
use crate::genome::gene::Gene;
use nalgebra::Vector2;

pub mod gene;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Genome {
    pub genes: [Option<Gene>; COUNT_GENES],
    pub step: usize,
    pub protection: f32,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            genes: [None; COUNT_GENES],
            step: 0,
            protection: 1.0,
        }
    }

    pub fn next_step(&mut self) {
        if self.step + 1 < COUNT_GENES {
            self.step += 1;
        } else {
            self.step = 0;
        }
    }

    pub fn current_gene(&self) -> Option<Gene> {
        self.genes[self.step]
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            genes: [
                Some(Gene::ResourceExtraction(
                    crate::resource::Resource::Chemosynthesis,
                )),
                Some(Gene::VectorMove(Vector2::new(3.0, 0.0))),
                Some(Gene::ResourceExtraction(
                    crate::resource::Resource::Chemosynthesis,
                )),
                Some(Gene::Reproduction),
                Some(Gene::VectorMove(Vector2::new(6.0, 6.0))),
                Some(Gene::ResourceExtraction(
                    crate::resource::Resource::Chemosynthesis,
                )),
                Some(Gene::VectorMove(Vector2::new(-4.0, 0.0))),
                Some(Gene::ResourceExtraction(
                    crate::resource::Resource::Chemosynthesis,
                )),
                Some(Gene::VectorMove(Vector2::new(-1.0, -3.0))),
                Some(Gene::ResourceExtraction(
                    crate::resource::Resource::Chemosynthesis,
                )),
                Some(Gene::Reproduction),
                None,
            ],
            ..Self::new()
        }
    }
}
