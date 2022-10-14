use std::fmt::Debug;

use crate::rnglib::*;

pub struct GeneticAlgorithm<T>
where T: Debug + Clone
{
    pub data_generator: fn(XorRng64) -> T,
    pub fitness_function: fn(&T) -> u64,
    pub crossover_function: fn(&T, &T) -> T,
    pub mutation_function: fn(XorRng64, &T) -> T,
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
            let num_to_crossover = num_pop / 2;
            let indices_to_crossover: Vec<u64> = (0..num_to_crossover).map(|n| rng.next(num_to_crossover)).collect();
            for i in 0..num_to_crossover{
                let crossed_genome = (self.crossover_function)(&genome_data[i as usize].0, &genome_data[indices_to_crossover[i as usize] as usize].0);
                new_population.push(crossed_genome);
            }

            // mutate any of them
            let num_to_mutate = num_pop - num_pop / 2;
            let indices_to_mutate: Vec<u64> = (0..num_to_mutate).map(|n| rng.next(num_pop)).collect();
            for i in 0..num_to_mutate{
                let mutated_genome = (self.mutation_function)(rng_generator.next(), &genome_data[indices_to_mutate[i as usize] as usize].0);
                new_population.push(mutated_genome);
            }

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