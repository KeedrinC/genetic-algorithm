use std::fmt::Debug;
use rand::{Rng, prelude::SliceRandom};
use crate::{SelectionMethod, CrossoverMethod, chromosome::Chromosome, MutationMethod, Individual};
pub struct SimpleSelection;
impl SelectionMethod for SimpleSelection {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where I: Individual + Debug {
        // The more fit the individual,
        population.to_owned()
            .choose_weighted(rng, |i| i.fitness())
            .expect("population is empty")
    }
}

pub struct SimpleCrossover;
impl CrossoverMethod for SimpleCrossover {
    fn crossover(&self, rng: &mut dyn rand::RngCore, a: &Chromosome, b: &Chromosome) -> Chromosome {
        let genes = a.iter().zip(b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) {a} else {b})
            .collect();
        Chromosome {genes}
    }
}

pub struct SimpleMutation {chance: f32, coeff: f32}
impl SimpleMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));
        Self { chance, coeff }
    }
}

impl MutationMethod for SimpleMutation {
    fn mutate(&self, rng: &mut dyn rand::RngCore, chromosome: &mut Chromosome) {
        for gene in chromosome.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}