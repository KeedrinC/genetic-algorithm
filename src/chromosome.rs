use std::ops::{Deref, DerefMut, RangeInclusive};
use rand::{*};

#[derive(Debug)]
pub struct Chromosome {pub genes: Vec<f32>}

impl Deref for Chromosome {
    type Target = Vec<f32>;
    fn deref(&self) -> &Self::Target {&self.genes}
}

impl DerefMut for Chromosome {
    fn deref_mut(&mut self) -> &mut Self::Target {&mut self.genes}
}

impl Chromosome {
    fn new(length: usize, range: Option<RangeInclusive<f32>>) -> Self {
        let mut n: rand::rngs::ThreadRng = rand::thread_rng();
        let genes: Vec<f32> = std::iter::repeat_with(|| {
            if let Some(range) = range.to_owned()
            {n.gen_range(range)} else {n.gen()}})
            .take(length)
            .collect();
        Self {genes}
    }
}

#[cfg(test)]
mod chromosome_tests {
    use super::*;
    fn chromosome() -> Chromosome {
        Chromosome {genes: vec![3.0, 1.0, 2.0]}
    }
    #[test]
    fn creation() {
        let chromosome = Chromosome::new(5, None);
        println!("created chromosome: {:#?}", chromosome);
        assert_eq!(chromosome.len(), 5);
    }
    #[test]
    fn iter_and_indexing() {
        let mut chromosome: Chromosome = chromosome();
        assert_eq!(chromosome.len(), 3);
        assert_eq!(chromosome[0], 3.0);
        assert_eq!(chromosome[1], 1.0);
        assert_eq!(chromosome[2], 2.0);

        let genes: Vec<&f32> = chromosome.iter().collect();
        assert_eq!(genes[0], &3.0);
        assert_eq!(genes[1], &1.0);
        assert_eq!(genes[2], &2.0);

        chromosome.iter_mut().for_each(|g| *g += 1.);
        assert_eq!(chromosome[0], 4.0);
        assert_eq!(chromosome[1], 2.0);
        assert_eq!(chromosome[2], 3.0);
    }
}