use rand::{Rng, thread_rng};
use crate::components::DNA;

pub struct GeneHandler {
    gene_container: Vec<[f32;100]>,
    score_records: Vec<f32>,
    pub generation: usize,
}

impl Default for GeneHandler {
    fn default() -> Self {
        let mut gene_container = vec![];
        let mut score = vec![];
        for _ in 0..100 {
            let mut genes = [0f32;100];
            thread_rng().fill(&mut genes[..]);
            gene_container.push(genes);
            score.push(-1.0f32);
        }
        GeneHandler{
            gene_container,
            score_records: score,
            generation : 0
        }
    }
}

impl GeneHandler {
    pub fn get_dna(&mut self) -> DNA{
        let mut index = 0;
        if let Some((found_index, _)) = self.score_records.iter().enumerate().find(
            |(_, &value)| value == -1.0) {
                index = found_index;
                self.score_records[index] = 0.0;
            }


        let genes = self.gene_container[index];
        DNA{
            hidden_layers: [6,4],
            genes,
            index,
        }
    }


    pub fn set_score(&mut self, index: usize, score:f32){
        self.score_records[index] = score;
    }


    pub fn process_generation(&mut self){
        let mut indexed_score: Vec<(usize, f32)> = self.score_records.iter().cloned().enumerate().collect();
        indexed_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let sorted_indices: Vec<usize> = indexed_score.iter().map(|(idx, _)| *idx).collect();


        for index in sorted_indices.iter().skip(10) {
            let mut genes = [0f32;100];
            thread_rng().fill(&mut genes[..]);
            self.gene_container[*index] = genes;
        }

        for score in self.score_records.iter_mut() {
            *score = -1.0f32;
        }
        self.generation += 1;
    }
}