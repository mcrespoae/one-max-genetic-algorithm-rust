#[path = "../src/one_max_genetic_algorithm.rs"]
mod one_max_genetic_algorithm;
use one_max_genetic_algorithm::*;


#[cfg(test)]

mod unit_tests {
    use super::*;

    #[test]
    fn test_random_genome_length() {
        let length = 10;
        let actual_genome = random_genome(length);
        assert_eq!(actual_genome.len(), length);
    }

    #[test]
    fn test_random_genome_range() {
        let length = 10;
        let genome = random_genome(length);
        for &gen in genome.iter() {
            assert!(gen == 0 || gen == 1);
        }
    }

    #[test]
    fn test_random_genome_zero_length() {
        let length = 0;
        let genome = random_genome(length);
        assert!(genome.is_empty());
    }

    #[test]
    fn test_init_population_length() {
        let population_size = 10;
        let genome_length = 5;
        let population = init_population(population_size, genome_length);
        assert_eq!(population.len(), population_size);
    }

    #[test]
    fn test_init_population_genome_length() {
        let population_size = 10;
        let genome_length = 5;
        let population = init_population(population_size, genome_length);
        for genome in &population {
            assert_eq!(genome.len(), genome_length);
        }
    }
    #[test]
    fn test_genome_fitness_all_zeros() {
        let genome = vec![0u8; 10];
        let expected_fitness = 0.0;
        let actual_fitness = get_genome_fitness(&genome);
        assert_eq!(actual_fitness, expected_fitness);
    }

    #[test]
    fn test_genome_fitness_all_ones() {
        let genome = vec![1u8; 10];
        let expected_fitness = 1.0;
        let actual_fitness = get_genome_fitness(&genome);
        println!("{actual_fitness}");

        assert_eq!(actual_fitness, expected_fitness);
    }

    #[test]
    fn test_genome_fitness_mixed() {
        let genome = vec![0, 1, 0, 1, 0, 1, 1, 0];
        let expected_fitness = 0.5;
        let actual_fitness = get_genome_fitness(&genome);
        assert_eq!(actual_fitness, expected_fitness);
    }

    #[test]
    fn test_calculate_population_fitness_empty_population() {
        let population: Vec<Vec<u8>> = Vec::new();
        let expected_fitness: Vec<f64> = Vec::new();
        let actual_fitness = calculate_population_fitness(&population);
        assert_eq!(actual_fitness, expected_fitness);
    }

    #[test]
    fn test_calculate_population_fitness_single_genome() {
        let population: Vec<Vec<u8>> = vec![vec![0, 1, 0, 1, 0, 1]];
        let expected_fitness = vec![0.5];
        let actual_fitness = calculate_population_fitness(&population);
        assert_eq!(actual_fitness, expected_fitness);
    }

    #[test]
    fn test_calculate_population_fitness_multiple_genomes() {
        let population: Vec<Vec<u8>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 0],
        ];
        let expected_fitness = vec![0.0, 0.6, 0.4];
        let actual_fitness = calculate_population_fitness(&population);
        assert_eq!(actual_fitness, expected_fitness);
    }

    #[test]
    fn test_get_target_fitness() {
        let expected_target_fitness: f64 = 1.0;
        let actual_target_fitness = get_target_fitness();
        assert_eq!(actual_target_fitness, expected_target_fitness);
    }

    #[test]
    fn test_get_best_fitness_empty_vector() {
        let fitness_values: Vec<f64> = Vec::new();
        let expected_best_fitness = f64::NEG_INFINITY;
        let actual_best_fitness = get_best_fitness(&fitness_values);
        assert_eq!(actual_best_fitness, expected_best_fitness);
    }

    #[test]
    fn test_get_best_fitness_single_value() {
        let fitness_values: Vec<f64> = vec![0.5];
        let expected_best_fitness = 0.5;
        let actual_best_fitness = get_best_fitness(&fitness_values);
        assert_eq!(actual_best_fitness, expected_best_fitness);
    }

    #[test]
    fn test_get_best_fitness_multiple_values() {
        let fitness_values: Vec<f64> = vec![0.2, 0.9, 0.4, 0.6, 0.3];
        let expected_best_fitness = 0.9;
        let actual_best_fitness = get_best_fitness(&fitness_values);
        assert_eq!(actual_best_fitness, expected_best_fitness);
    }

    #[test]
    fn test_get_generation_fitness_empty_population() {
        let fitness_values: Vec<f64> = Vec::new();
        let expected_generation_fitness = 0.0; // Since the population is empty, the generation fitness should be 0
        let actual_generation_fitness = get_generation_fitness(&fitness_values, 0);
        assert_eq!(actual_generation_fitness, expected_generation_fitness);
    }

    #[test]
    fn test_get_generation_fitness_single_value_population() {
        let fitness_values: Vec<f64> = vec![0.8];
        let expected_generation_fitness = 0.8; // Single value in the population, generation fitness should be that value
        let actual_generation_fitness = get_generation_fitness(&fitness_values, 1);
        assert_eq!(actual_generation_fitness, expected_generation_fitness);
    }

    #[test]
    fn test_get_generation_fitness_multiple_values_population() {
        let fitness_values: Vec<f64> = vec![0.5, 0.7, 0.9];
        let expected_generation_fitness = 0.7; // Average of values in the population
        let actual_generation_fitness = get_generation_fitness(&fitness_values, 3);
        let epsilon = 0.0001; // Define a small epsilon value
        assert!((actual_generation_fitness - expected_generation_fitness).abs() < epsilon);
    }


    #[test]
    fn test_select_parent() {
        // Test scenario with a small population
        let population = vec![
            vec![1, 1, 1, 1],  // Individual 1
            vec![1, 0, 1, 0],  // Individual 2
            vec![0, 0, 1, 1],  // Individual 3
            vec![0, 0, 0, 0],  // Individual 4
        ];
        let fitness_values = calculate_population_fitness(&population);

        let mode = "gibberish";
        let selected_individual = select_parent(&population, &fitness_values, mode);
        assert!(population.contains(&selected_individual)); // Ensure the selected individual is from the population
    }


    #[test]
    fn test_select_parent_tournament() {
        // Test scenario with a small population
        let population = vec![
            vec![1, 1, 1, 1],  // Individual 1
            vec![1, 0, 1, 0],  // Individual 2
            vec![0, 0, 1, 1],  // Individual 3
            vec![0, 0, 0, 0],  // Individual 4
        ];
        let fitness_values = calculate_population_fitness(&population);
        let tournament_size = 2;

        let selected_individual = select_parent_tournament(&population, &fitness_values, tournament_size);
        assert!(population.contains(&selected_individual)); // Ensure the selected individual is from the population
    }

    #[test]
    fn test_select_parent_roulette() {
        // Test scenario with a small population
        let population = vec![
            vec![1, 1, 1, 1],  // Individual 1
            vec![1, 0, 1, 0],  // Individual 2
            vec![0, 0, 1, 1],  // Individual 3
            vec![0, 0, 0, 0],  // Individual 4
        ];
        let fitness_values = calculate_population_fitness(&population);

        let selected_individual = select_parent_roulette(&population, &fitness_values);
        assert!(population.contains(&selected_individual)); // Ensure the selected individual is from the population
    }

    #[test]
    fn test_crossover_no_crossover() {
        // Given
        let parent1 = vec![1, 2, 3, 4, 5];
        let parent2 = vec![6, 7, 8, 9, 10];
        let crossover_rate = 0.0; // No crossover


        let (child1, child2) = crossover(&parent1, &parent2, crossover_rate);


        assert_eq!(child1, parent1);
        assert_eq!(child2, parent2);
    }

    #[test]
    fn test_crossover_with_crossover() {
        // Given
        let parent1 = vec![1, 2, 3, 4, 5];
        let parent2 = vec![6, 7, 8, 9, 10];
        let crossover_rate = 1.0; // Always crossover

        let (child1, child2) = crossover(&parent1, &parent2, crossover_rate);

        // Check that the lengths of the children are the same as the parents
        assert_eq!(child1.len(), parent1.len());
        assert_eq!(child2.len(), parent2.len());

        // Check that the new elements are different
        assert_ne!(child1, parent1);
        assert_ne!(child2, parent2);
        assert_ne!(child1, child2);


        }

    #[test]
    fn test_mutate_zeros_with_rate_1() {
        assert_eq!(mutate(&[0, 0, 0, 0], 1.0), vec![1, 1, 1, 1]);
    }
    #[test]
    fn test_mutate_ones_with_rate_1() {
        assert_eq!(mutate(&[1, 1, 1, 1], 1.0), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_mutate_mixed_with_rate_0() {
        assert_eq!(mutate(&[0, 1, 0, 1], 0.0), vec![0, 1, 0, 1]);
    }

    #[test]
    fn test_mutate_mixed_with_rate_0_5() {
        let mutated_genome = mutate(&[0, 1, 0, 1], 0.5);
        assert_eq!(mutated_genome.len(), 4);
    }


    #[test]
    fn test_mutate_population_fitness_all_ones() {
        let num_times = 100_000;
        let mutation_rate = 0.5;
        let genome_length = 50;
        let mut mutated_gen_fitness = Vec::new();
        let genome = vec![1; genome_length];
        for _ in 0..num_times {
            let mutated_genome = mutate(&genome, mutation_rate);
            mutated_gen_fitness.push(get_genome_fitness(&mutated_genome));
        }

        let avg_mutation_gen_fitness: f64 = mutated_gen_fitness.iter().sum::<f64>() / mutated_gen_fitness.len() as f64;
        assert!(0.49 <= avg_mutation_gen_fitness && avg_mutation_gen_fitness <= 0.51);
    }

    #[test]
    fn test_mutate_population_fitness_all_zeroes() {
        let num_times = 100_000;
        let mutation_rate = 0.5;
        let genome_length = 50;
        let mut mutated_gen_fitness = Vec::new();
        let genome = vec![0; genome_length];

        for _ in 0..num_times {
            let mutated_genome = mutate(&genome, mutation_rate);
            mutated_gen_fitness.push(get_genome_fitness(&mutated_genome));
        }

        let avg_mutation_gen_fitness: f64 = mutated_gen_fitness.iter().sum::<f64>() / mutated_gen_fitness.len() as f64;
        assert!(0.49 <= avg_mutation_gen_fitness && avg_mutation_gen_fitness <= 0.51);
    }

    #[test]
    fn test_population_size_odd() {
        let population_size = 101;
        let population: Vec<Vec<u8>> = vec![vec![0, 1, 0, 1]; population_size];
        let fitness_values: Vec<f64> = vec![0.5; population_size];
        let select_parent_mode = "tournament";
        let crossover_rate = 0.8;
        let mutation_rate = 0.03;

        let population_fitness = calculate_population_fitness(&population);
        let population_fitness_avg = get_generation_fitness(&population_fitness, population_size);

        let new_population = create_new_population(
            population_size,
            &population,
            &fitness_values,
            select_parent_mode,
            crossover_rate,
            mutation_rate,
        );

        let new_population_fitness = calculate_population_fitness(&new_population);
        let new_population_fitness_avg = get_generation_fitness(&new_population_fitness, population_size);

        assert_eq!(new_population.len(), population_size);
        assert!(population_fitness_avg * 0.8 <= new_population_fitness_avg);
    }

    #[test]
    fn test_genetic_algorithm_custom_parameters() {
        // Test with custom parameters
        let population_size = 50;
        let genome_length = 30;
        let max_generations = 500;
        let mutation_rate = 0.01;
        let crossover_rate = 0.8;
        let select_parent_mode = "roulette";
        let target_generation_fitness = 0.95;
        let verbose = true;

        let (generation, generation_fitness, best_fitness) = genetic_algorithm(
            population_size,
            genome_length,
            max_generations,
            mutation_rate,
            crossover_rate,
            select_parent_mode,
            target_generation_fitness,
            verbose,
        );


        // Check if generation is less than or equal to max_generations
        assert!(generation <= max_generations);

        // Check if generation_fitness and best_fitness are within the range [0, 1]
        assert!(generation_fitness >= 0.0 && generation_fitness <= 1.0);
        assert!(best_fitness >= 0.0 && best_fitness <= 1.0);
    }

}



mod integration_tests {
    use super::*;
    #[test]
    fn test_inte_new_population_random() {
        // Test with custom parameters
        let population_size = 50;
        let genome_length = 20;
        let num_times = 200;

        let mut random_fitness = Vec::new();
        for _ in 0..num_times {
            let population = init_population(population_size, genome_length);
            let fitness_values = calculate_population_fitness(&population);
            let generation_fitness = get_generation_fitness(&fitness_values, population_size);
            random_fitness.push(generation_fitness);
        }

        let fitness_avg: f64 = random_fitness.iter().sum::<f64>() / random_fitness.len() as f64;
        assert!(0.49 <= fitness_avg && fitness_avg <= 0.51);
    }

    #[test]
    fn test_inte_new_population_zeroes() {
        // Test with custom parameters
        let population_size = 1_000;
        let genome_length = 50;
        let crossover_rate = 0.2;
        let mutation_rate = 0.03;

        let genome = vec![0; genome_length];
        let population = vec![genome.clone(); population_size];
        let fitness_values = calculate_population_fitness(&population);
        let mut new_population = Vec::new();

        for _ in 0..population_size / 2 {
            let parent1 = select_parent(&population, &fitness_values, "tournament");
            let parent2 = select_parent(&population, &fitness_values, "roulette");
            let (offspring1, offspring2) = crossover(&parent1, &parent2, crossover_rate);
            new_population.push(mutate(&offspring1, mutation_rate));
            new_population.push(mutate(&offspring2, mutation_rate));
        }
        if population_size % 2 != 0{
            let parent = select_parent(&population, &fitness_values, "roulette");
            new_population.push(mutate(&parent, mutation_rate));
        }

        let fitness_values = calculate_population_fitness(&new_population);
        let generation_fitness = get_generation_fitness(&fitness_values, population_size);

        assert!(0.02 <= generation_fitness && generation_fitness <= 0.04);
    }

    #[test]
    fn test_inte_get_avg() {
        // Test with custom parameters
        let population_size = 1_000;
        let genome_length = 20;
        let crossover_rate = 0.6;
        let mutation_rate = 0.07;
        let num_times = 10;

        let genome = vec![0u8; genome_length];
        let population = vec![genome.clone(); population_size];
        let mut generation_fitness = Vec::new();
        let mut best_genome_fitness = Vec::new();

        for _ in 0..num_times {
            let new_population = create_new_population(
                population_size,
                &population,
                &calculate_population_fitness(&population),
                "roulette",
                crossover_rate,
                mutation_rate,
            );
            let fitness_values = calculate_population_fitness(&new_population);
            generation_fitness.push(get_generation_fitness(&fitness_values, population_size));
            best_genome_fitness.push(get_best_fitness(&fitness_values));
        }

        let avg_best_fitness: f64 = generation_fitness.iter().sum::<f64>() / num_times as f64;
        let avg_best_genome_fitness: f64 = best_genome_fitness.iter().sum::<f64>() / num_times as f64;
        assert!(0.06 <= avg_best_fitness && avg_best_fitness <= 0.073);
        assert!(0.25 <= avg_best_genome_fitness && avg_best_genome_fitness <= 0.45);
    }
}
