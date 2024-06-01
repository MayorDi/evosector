use crate::constants::COUNT_GENES;
use crate::genome::gene::Gene;
use crate::resource::Resource;
use crate::traits::Mutable;
use nalgebra::Vector2;
use rand::Rng;

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

impl Mutable for Genome {
    fn mutate(&mut self) -> bool {
        if rand::thread_rng().gen_bool(0.05) {
            let idx_mutate = rand::thread_rng().gen_range(0..COUNT_GENES);
            let new_gene_mutate = rand::thread_rng().gen_range(0..6);
            let gene = &mut self.genes[idx_mutate];
            let move_vector = Vector2::new(
                rand::thread_rng().gen_range(-3.0..3.0),
                rand::thread_rng().gen_range(-3.0..3.0),
            );

            match new_gene_mutate {
                0 => *gene = Some(Gene::ProtectionMutate(0.0)),
                1 => *gene = Some(Gene::ProtectionBody(0.0)),
                2 => *gene = Some(Gene::VectorMove(move_vector)),
                3 => *gene = Some(Gene::Attack(0.0)),
                4 => *gene = Some(Gene::ResourceExtraction(Resource::Chemosynthesis)),
                5 => *gene = Some(Gene::Reproduction),
                _ => *gene = None,
            }

            return true;
        }

        false
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            genes: [
                Some(Gene::ResourceExtraction(Resource::Chemosynthesis)),
                Some(Gene::VectorMove(Vector2::new(2.0, 0.0))),
                Some(Gene::ResourceExtraction(Resource::Chemosynthesis)),
                Some(Gene::Reproduction),
                Some(Gene::VectorMove(Vector2::new(2.0, 2.0))),
                Some(Gene::ResourceExtraction(Resource::Photosynthesis)),
                Some(Gene::VectorMove(Vector2::new(-2.0, 0.0))),
                Some(Gene::ResourceExtraction(Resource::Chemosynthesis)),
                Some(Gene::VectorMove(Vector2::new(-2.0, -2.0))),
                Some(Gene::ResourceExtraction(Resource::Chemosynthesis)),
                Some(Gene::Reproduction),
                None,
            ],
            ..Self::new()
        }
    }
}
