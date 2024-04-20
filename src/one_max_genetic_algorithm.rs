use rand::seq::index::sample;
use rand::{thread_rng, Rng};

pub fn random_genome(length: usize) -> Vec<u8> {
    let mut genome: Vec<u8> = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        let gen: u8 = rng.gen_bool(0.5) as u8;
        genome.push(gen);
    }
    genome
}

pub fn init_population(population_size: usize, genome_length: usize) -> Vec<Vec<u8>> {
    let mut population: Vec<Vec<u8>> = Vec::with_capacity(population_size);
    for _ in 0..population_size {
        let genome: Vec<u8> = random_genome(genome_length);
        population.push(genome);
    }
    population
}

pub fn get_genome_fitness(genome: &[u8]) -> f64 {
    let sum: u32 = genome.iter().map(|&x| x as u32).sum();
    let genome_fitness: f64 = sum as f64 / genome.len() as f64;
    if genome_fitness.is_nan() {
        return 0.0; // If result is NaN, return 0 as genome fitness
    }
    genome_fitness
}

pub fn calculate_population_fitness(population: &Vec<Vec<u8>>) -> Vec<f64> {
    let mut population_fitness_vector: Vec<f64> = Vec::with_capacity(population.len());

    for genome in population {
        let genome_fitness: f64 = get_genome_fitness(genome);
        population_fitness_vector.push(genome_fitness);
    }
    population_fitness_vector
}

pub fn get_target_fitness() -> f64 {
    1.0
}

pub fn get_best_fitness(fitnesses_values: &[f64]) -> f64 {
    fitnesses_values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
}

pub fn get_generation_fitness(fitnesses_values: &[f64], population_size: usize) -> f64 {
    let mut sum_fitness: f64 = fitnesses_values.iter().sum();
    sum_fitness /= population_size as f64;
    if sum_fitness.is_nan() {
        return 0.0; // If result is NaN, return 0 as generation fitness
    }
    sum_fitness
}

pub fn select_parent(population: &Vec<Vec<u8>>, fitness_values: &[f64], mode: &str) -> Vec<u8> {
    if mode.to_lowercase() == "tournament"
        || !["roulette", "tournament"].contains(&mode.to_lowercase().as_str())
    {
        let mut tournament_size: usize = thread_rng().gen_range(
            ((*population).len() as f64 * 0.6) as usize
                ..=((*population).len() as f64 * 0.8) as usize,
        );
        tournament_size = if tournament_size > 1 {
            tournament_size
        } else {
            1
        };
        select_parent_tournament(population, fitness_values, tournament_size)
    } else {
        select_parent_roulette(population, fitness_values)
    }
}

pub fn select_parent_tournament(
    population: &[Vec<u8>],
    fitness_values: &[f64],
    tournament_size: usize,
) -> Vec<u8> {
    // Tournament implementation
    let mut rng = rand::thread_rng();
    let selected_indices = sample(&mut rng, population.len(), tournament_size);

    let winner = selected_indices
        .iter()
        .map(|i| (population[i].clone(), fitness_values[i]))
        .max_by(|(_, fitness1), (_, fitness2)| fitness1.partial_cmp(fitness2).unwrap())
        .unwrap(); // Select the candidate with the highest fitness
    winner.0 // Return the selected individual from the winning tournament
}

pub fn select_parent_roulette(population: &[Vec<u8>], fitness_values: &[f64]) -> Vec<u8> {
    let total_fitness: f64 = fitness_values.iter().sum();
    if total_fitness == 0.0 {
        return population[0].clone();
    }
    let pick = thread_rng().gen_range(0.0..total_fitness);
    let mut current = 0.0;
    for (individual, fitness_value) in population.iter().zip(fitness_values.iter()) {
        current += *fitness_value;
        if current > pick {
            return individual.clone();
        }
    }
    population[0].clone() // Default to the first individual if no individual is selected
}

pub fn crossover(parent1: &[u8], parent2: &[u8], crossover_rate: f64) -> (Vec<u8>, Vec<u8>) {
    let mut rng = thread_rng();
    let random_float: f64 = rng.gen_range(0.0..=1.0);
    let mut child1: Vec<u8> = Vec::with_capacity(parent1.len());
    let mut child2: Vec<u8> = Vec::with_capacity(parent2.len());

    if random_float < crossover_rate {
        let crossover_point = rng.gen_range(1..parent1.len());

        child1.extend_from_slice(&parent1[..crossover_point]);
        child1.extend_from_slice(&parent2[crossover_point..]);

        child2.extend_from_slice(&parent2[..crossover_point]);
        child2.extend_from_slice(&parent1[crossover_point..]);
    } else {
        return (parent1.to_vec(), parent2.to_vec());
    }
    (child1, child2)
}

pub fn mutate(genome: &[u8], mutation_rate: f64) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut mutated_genome = genome.to_vec();

    for gene in mutated_genome.iter_mut().take(genome.len()) {
        if rng.gen::<f64>() < mutation_rate {
            *gene ^= 1;
        }
    }
    mutated_genome
}

pub fn print_best_values(fitness_values: &[f64], population: &[Vec<u8>], generation_fitness: f64) {
    let best_index = fitness_values
        .iter()
        .position(|&x| x == get_best_fitness(fitness_values))
        .unwrap();
    let best_solution = &population[best_index];
    println!("Best Final Solution: {:?}", best_solution);
    println!(
        "Best Final Fitness: {:?}",
        get_genome_fitness(best_solution)
    );
    println!(
        "Generation perfect fitness percentage: {:.2}",
        generation_fitness
    );
}

pub fn create_new_population(
    population_size: usize,
    population: &Vec<Vec<u8>>,
    fitness_values: &[f64],
    select_parent_mode: &str,
    crossover_rate: f64,
    mutation_rate: f64,
) -> Vec<Vec<u8>> {
    let mut new_population = vec![];

    for _ in 0..population_size / 2 {
        let parent1 = select_parent(population, fitness_values, select_parent_mode);
        let parent2 = select_parent(population, fitness_values, select_parent_mode);
        let (offspring1, offspring2) = crossover(&parent1, &parent2, crossover_rate);
        new_population.push(mutate(&offspring1, mutation_rate));
        new_population.push(mutate(&offspring2, mutation_rate));
    }
    if population_size % 2 != 0 {
        let parent = select_parent(population, fitness_values, select_parent_mode);
        new_population.push(mutate(&parent, mutation_rate));
    }
    new_population
}

#[allow(clippy::too_many_arguments)]
pub fn genetic_algorithm(
    population_size: usize,
    genome_length: usize,
    max_generations: u32,
    mutation_rate: f64,
    crossover_rate: f64,
    select_parent_mode: &str,
    target_generation_fitness: f64,
    verbose: bool,
) -> (u32, f64, f64) {
    let target_fitness = get_target_fitness();
    let mut population = init_population(population_size, genome_length);
    let mut fitness_values = calculate_population_fitness(&population);

    let mut best_population = vec![];
    let mut best_generation = 0;
    let mut best_generation_fitness = 0.0;
    let mut best_fitness = 0.0;

    for generation in 0..max_generations {
        population = create_new_population(
            population_size,
            &population,
            &fitness_values,
            select_parent_mode,
            crossover_rate,
            mutation_rate,
        );
        fitness_values = calculate_population_fitness(&population);
        let generation_fitness = get_generation_fitness(&fitness_values, population_size);
        let best_gen_fitness = get_best_fitness(&fitness_values);

        if verbose {
            println!(
                "Generation {}: Best Fitness = {} Generation Fitness Percentage: {:.2}",
                generation, best_gen_fitness, generation_fitness
            );
        }

        if generation_fitness >= best_fitness {
            best_population = population.clone();
            best_generation = generation;
            best_generation_fitness = generation_fitness;
            best_fitness = best_gen_fitness;
        }

        if generation_fitness >= target_generation_fitness
            && (best_gen_fitness - target_fitness).abs() < f64::EPSILON
        {
            if verbose {
                println!("Ideal solution found in generation {}.", generation);
                print_best_values(&fitness_values, &population, generation_fitness);
            }
            return (generation, generation_fitness, best_fitness); // Early return
        }
    }

    if verbose {
        println!(
            "Best solution found after {} generations was generation number {}.",
            max_generations, best_generation
        );
        let best_fitness_values = calculate_population_fitness(&best_population);
        print_best_values(
            &best_fitness_values,
            &best_population,
            best_generation_fitness,
        );
    }
    (max_generations, best_generation_fitness, best_fitness)
}
