
mod rnglib;
use rnglib::*;

mod genetic_algo;
use genetic_algo::*;

fn main() {

    let algo = GeneticAlgorithm{
        data_generator,
        mutation_function,
        crossover_function,
        fitness_function,
        crossover_selection_threshold_percent: 50,
        crossover_resulting_percent: 50,
        mutation_selection_threshold_percent: 100,
        mutation_resulting_percent: 30,
        elitism_percent: 20
    };

    let rng_generator = RngGenerator64::new_time_seeded();

    let data = algo.run_n_times(rng_generator, 10, 10);

    println!("{:?}", data);
}

type Genome = [u64; 10];
const TARGET_VALUE: i64 = 30;
const BASKET: [i64; 10] = [6, 9, 10, 3, 15, 17, 40, 20, 50, 30];

fn fitness_function(genome: &Genome) -> u64{
    let genome_value: i64 = BASKET.iter().zip(genome).fold(0, |acc, (b, g)| acc + b*(*g as i64));
    return (TARGET_VALUE - genome_value).abs() as u64;
}

fn data_generator(mut rng: XorRng64) -> Genome{
    let mut new_genome = [0; 10];
    for i in 0..10{
        let randint = rng.next(10);
        if randint >= 5 {
            new_genome[i] = 1
        }
    }
    return new_genome;
}

fn mutation_function(mut rng: XorRng64, old_genome: &Genome) -> Genome{
    let mut new_genome = old_genome.clone();
    let i = rng.next(10);
    new_genome[i as usize] = if old_genome[i as usize] == 0 {1} else {0};
    return new_genome;
}

fn crossover_function(g1: &Genome, g2: &Genome) -> Genome{
    let mut new_genome = [0; 10];
    for i in 0..5{
        new_genome[i] = g1[i];
    }
    for i in 5..10{
        new_genome[i] = g2[i];
    }
    return new_genome;
}