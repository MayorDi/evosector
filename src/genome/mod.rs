use std::ops::{Index, IndexMut};

use rand::Rng;

use crate::constants::{COUNT_GENES, PROBABILITY_OF_MUTATION};
use crate::genome::gene::Gene;
use crate::traits::Mutable;

pub mod gene;

#[derive(Debug, Clone, PartialEq)]
pub struct Genome {
    pub genes: [Option<Gene>; COUNT_GENES],
    pub step: usize,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            genes: [None; COUNT_GENES],
            step: 0,
        }
    }
}

impl Index<usize> for Genome {
    type Output = Option<Gene>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl IndexMut<usize> for Genome {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.genes[index]
    }
}

impl Mutable for Genome {
    fn mutate(&mut self) -> bool {
        if rand::thread_rng().gen_range(0.0..1.0) < PROBABILITY_OF_MUTATION {
            for gene in self.genes.iter_mut() {
                match gene {
                    Some(gene) => gene.mutate(),
                    None => {
                        *gene = Some(Gene::from(
                            rand::thread_rng().gen_range(0..Gene::VARIANT_COUNT),
                        ));
                        gene.unwrap().mutate()
                    }
                };
            }

            return true;
        }

        false
    }
}

impl Default for Genome {
    fn default() -> Self {
        let genome = Genome::new();

        genome
    }
}
