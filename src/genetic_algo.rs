use std::{fmt::Debug, f64::MAX_10_EXP};

use crate::rnglib::*;

pub struct GeneticAlgorithm<T>
where T: Debug + Clone
{
    pub data_generator: fn(XorRng64) -> T,
    pub fitness_function: fn(&T) -> u64,
    pub crossover_function: fn(&T, &T) -> T,
    pub mutation_function: fn(XorRng64, &T) -> T,
    pub crossover_selection_threshold_percent: u64,     // the percentile above which genomes will be used for crossover
    pub crossover_resulting_percent: u64,               // the percentile of new populations which will be from crossover
    pub mutation_selection_threshold_percent: u64,
    pub mutation_resulting_percent: u64,
    pub elitism_percent: u64,                           // the percent of highest performers which will be copied directly into new populations
}
impl<T> GeneticAlgorithm<T>
where T: Debug + Clone
{

    pub fn generate_initial_data(&self, num: u64) -> Vec<T>{
        let mut initial_population = Vec::new();

        let mut rng_generator = RngGenerator64::new_time_seeded();


        for _ in 0..num{
            let rng = rng_generator.next();
            let new_member = (self.data_generator)(rng);
            initial_population.push(new_member);
        }

        return initial_population;
    }

    pub fn run_n_times(&self, mut rng_generator: RngGenerator64, num_pop: u64, num_runs: u64) -> (T, u64){

        let initial_population = self.generate_initial_data(num_pop);

        let mut current_population = initial_population;

        let mut i = 0;

        let mut rng = rng_generator.next();

        let mut genome_data: Vec<(T, u64)>;

        let final_population = loop{
            // measure fitness of entire population
            let fitnesses = current_population.iter().map(self.fitness_function).collect::<Vec<u64>>();
            genome_data = current_population.into_iter().zip(fitnesses).collect();
            genome_data.sort_by_key(|(genome, fitness)| *fitness);

            //println!("{:?}", &genome_data);
            // check if we have a winner
            match genome_data[0]{
                (_, 0) => break genome_data,
                _ => ()
            }

            let mut new_population = Vec::new();
            // select the highest performing 50% and cross them over randomly
            let num_to_crossover = (self.crossover_resulting_percent * num_pop) / 100;
            let max_index_to_crossover = (self.crossover_selection_threshold_percent * num_pop) / 100;
            let indices_to_crossover: Vec<u64> = (0..num_to_crossover).map(|n| rng.next(max_index_to_crossover)).collect();
            for i in 0..num_to_crossover{
                let crossed_genome = (self.crossover_function)(&genome_data[i as usize].0, &genome_data[indices_to_crossover[i as usize] as usize].0);
                new_population.push(crossed_genome);
            }

            // mutate any of them
            let num_to_mutate = (self.mutation_resulting_percent * num_pop) / 100;
            let max_index_to_mutate = (self.mutation_selection_threshold_percent * num_pop) / 100;
            println!("mut: {} * {} / 100 = {}", self.mutation_selection_threshold_percent, num_pop, max_index_to_mutate);
            let indices_to_mutate: Vec<u64> = (0..num_to_mutate).map(|n| rng.next(max_index_to_mutate)).collect();
            for i in 0..num_to_mutate{
                let mutated_genome = (self.mutation_function)(rng_generator.next(), &genome_data[indices_to_mutate[i as usize] as usize].0);
                new_population.push(mutated_genome);
            }

            println!("{} crossed, {} mutated", num_to_crossover, num_to_mutate);
            // elitism
            let num_for_elitism = (self.elitism_percent * num_pop) / 100;
            println!("{}", num_for_elitism);
            for i in 0..num_for_elitism{
                new_population.push(genome_data[i as usize].0.clone())
            }

            assert_eq!(num_pop, num_to_crossover + num_to_mutate + num_for_elitism, "Mismatching population sizes");

            // loop cleanup
            i += 1;
            if i == num_runs{
                break genome_data;
            }
            
            current_population = new_population;
        };

        let winning_genome = final_population[0].0.clone();
        return (winning_genome, final_population[0].1);

    }

}