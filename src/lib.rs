use std::fmt::Debug;
use rand::{self, rngs::{StdRng}, SeedableRng};
use crate::{
    chromosome::Chromosome,
    statistics::Statistics,
};

mod chromosome;
mod simple_methods;
mod statistics;

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}

trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I where
        I: Individual + Debug;
}

trait MutationMethod {
    fn mutate(&self, rng: &mut dyn rand::RngCore, chromosome: &mut Chromosome);
}

trait CrossoverMethod {
    fn crossover(&self, rng: &mut dyn rand::RngCore, a: &Chromosome, b: &Chromosome) -> Chromosome;
}

struct GeneticAlgorithm<S, C, M>  {
    rng: Box<dyn rand::RngCore>,
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M> where
S: SelectionMethod, C: CrossoverMethod, M: MutationMethod {
    fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        let rng: Box<StdRng> = Box::new(rand::rngs::StdRng::from_seed(Default::default()));
        Self {selection_method, crossover_method, mutation_method, rng}
    }
    fn evolve<I>(&mut self, population: &[I]) -> Vec<I> where
    I: Individual + Debug {
        let mut individuals = Vec::new();
        for _ in 0..population.len() {
            let chromosomes: [&Chromosome; 2] = [
                self.selection_method.select(&mut self.rng, population).chromosome(),
                self.selection_method.select(&mut self.rng, population).chromosome()];
            let mut chromosome: Chromosome = self.crossover_method.crossover(
                &mut self.rng, chromosomes[0], chromosomes[1]);
            self.mutation_method.mutate(&mut self.rng, &mut chromosome);
            individuals.push(I::create(chromosome));
        }
        individuals
    }
}

#[cfg(test)]
mod genetic_algorithm_tests {
    use super::*;
    use crate::simple_methods::{SimpleSelection, SimpleCrossover, SimpleMutation};
    fn chromosome(genes: Vec<f32>) -> Chromosome {
        Chromosome {genes}
    }

    #[derive(Debug)]
    struct Entity {chromosome: Chromosome}
    impl Individual for Entity {
        fn create(chromosome: Chromosome) -> Self {Self {chromosome}}
        fn chromosome(&self) -> &Chromosome {&self.chromosome}
        fn fitness(&self) -> f32 {
            let sum: f32 = self.chromosome().iter().sum();
            return sum;
        }
    }

    #[test]
    fn test() {
        let mut algorithm: GeneticAlgorithm<_, _, _> = GeneticAlgorithm::new(
            SimpleSelection,
            SimpleCrossover,
            SimpleMutation::new(0.5, 0.5));
        let mut population = vec![
            Entity::create(chromosome(vec![0.0, 0.0, 0.0])),
            Entity::create(chromosome(vec![0.0, 0.0, 1.0])),
            Entity::create(chromosome(vec![0.0, 1.0, 1.0])),
            Entity::create(chromosome(vec![1.0, 1.0, 1.0])),
            Entity::create(chromosome(vec![1.0, 1.0, 2.0])),
            Entity::create(chromosome(vec![1.0, 2.0, 2.0])),
            Entity::create(chromosome(vec![2.0, 2.0, 2.0])),
        ];
        let evolutions: usize = 20;
        let mut total: usize = 0;
        let chunk = 20;
        println!("{:#?}", Statistics::new(&population));
        for _ in 0..chunk {
            population = algorithm.evolve(&population);
            total += 1;
            println!("Evolutions: {total} -> {:#?}", Statistics::new(&population));
        }
    }
}