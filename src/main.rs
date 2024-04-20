use std::sync::{Arc, Mutex};
use std::thread;
extern crate num_cpus;

use tqdm::pbar;

mod utils;
use utils::{distribute_run_times, generate_equally_spaced_values};

mod one_max_genetic_algorithm;
use one_max_genetic_algorithm::genetic_algorithm;

mod results;
use results::Results;

// Constants
const RUN_TIMES: usize = 10;
const GENERATIONS: u32 = 500;
const POPULATION_SIZE: usize = 90;
const GENOME_LENGTH: usize = 25;
const SELECT_PARENT_MODE: &str = "tournament"; // tournament or roulette. Tournament usually converges faster and yields better results.
const TARGET_GENERATION_FITNESS: f64 = 0.995; // When a generation is considered fit enough to skip the next iterations. Values close to 1.0 will yield better results.
const TARGET_PROBLEM_FITNESS: f64 = 0.999; // When the problem is marked as solved. Values very close to 1.0 will not stop the execution.
const MUTATION_RATE_MIN: f64 = 0.001;
const MUTATION_RATE_MAX: f64 = 0.01;
const CROSSOVER_RATE_MIN: f64 = 0.1;
const CROSSOVER_RATE_MAX: f64 = 0.6;

pub fn process_genetic_algorithm(mutation_rate_values: &[f64], crossover_rate_values: &[f64]) {
    let max_threads: usize = num_cpus::get() - 2; // Use max_cpu count - 2 for maximum concurrent threads
                                                  // Calculate the number of threads of each iteration based on MAX_THREADS
    let distributed_run_times: Vec<usize> = distribute_run_times(max_threads, RUN_TIMES);

    let total_iterations = mutation_rate_values.len() * crossover_rate_values.len();
    let mut pbar = pbar(Some(total_iterations));

    let mut best_mutation_rate: f64 = 0.0;
    let mut best_crossover_rate: f64 = 0.0;
    let mut prev_best_score: f64 = 0.0;

    let mut best_result: Results = Results::new(GENERATIONS, 1.0);
    let mut score: f64;

    for &mutation_rate in mutation_rate_values {
        let mut prev_local_score: f64 = 0.0;

        for (i, &crossover_rate) in crossover_rate_values.iter().enumerate() {
            let result = Arc::new(Mutex::new(Results::new(GENERATIONS, 1.0)));
            let distributed_run_times_clone = distributed_run_times.clone();
            // Outer loop to control the number of iterations based on MAX_THREADS
            for local_run_times in distributed_run_times_clone {
                let mut handles = vec![];

                for _ in 0..local_run_times {
                    let result_clone = Arc::clone(&result);
                    let handle = thread::spawn(move || {
                        let (generation, generation_fitness, best_fitness) = genetic_algorithm(
                            POPULATION_SIZE,
                            GENOME_LENGTH,
                            GENERATIONS,
                            mutation_rate,
                            crossover_rate,
                            SELECT_PARENT_MODE,
                            TARGET_GENERATION_FITNESS,
                            false,
                        );
                        let mut result = result_clone.lock().unwrap();
                        result.add_result(generation, generation_fitness, best_fitness);
                    });

                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            }

            let result = result.lock().unwrap();
            score = result.get_score();

            // General score check
            if score >= prev_best_score {
                best_mutation_rate = mutation_rate;
                best_crossover_rate = crossover_rate;
                best_result = result.clone();
                prev_best_score = score;
            }

            if prev_local_score < score && i != 0 {
                // Skip this loop since the score is not improving
                pbar.update(crossover_rate_values.len() - i).unwrap();
                break;
            }

            prev_local_score = score;
            pbar.update(1).unwrap();

            if prev_best_score >= TARGET_PROBLEM_FITNESS {
                // Check if perfect score to close the algorithm execution
                break;
            }
        }
    }

    pbar.close().unwrap();
    println!("--------------------------------------------------");
    println!("\tBest results");
    println!("--------------------------------------------------");
    println!(
        "Best Mutation Rate: {best_mutation_rate}\nBest Crossover Rate: {best_crossover_rate}"
    );
    best_result.print();
}

fn main() {
    let mutation_rate_values: Vec<f64> =
        generate_equally_spaced_values(MUTATION_RATE_MIN, MUTATION_RATE_MAX, 8, true);
    let crossover_rate_values: Vec<f64> =
        generate_equally_spaced_values(CROSSOVER_RATE_MIN, CROSSOVER_RATE_MAX, 5, false);

    println!(
        "Running {} times the one max problem with genetic algorithms for:
    Generations:           {}
    Population Size:       {}
    Genome Length:         {}
    Parent selection mode: {}
    Mutation Rate:         {:.4} to {:.4}
    Crossover Rate:        {:.4} to {:.4}",
        RUN_TIMES,
        GENERATIONS,
        POPULATION_SIZE,
        GENOME_LENGTH,
        SELECT_PARENT_MODE,
        mutation_rate_values.first().unwrap_or(&0.0),
        mutation_rate_values.last().unwrap_or(&0.0),
        crossover_rate_values.first().unwrap_or(&0.0),
        crossover_rate_values.last().unwrap_or(&0.0),
    );
    process_genetic_algorithm(&mutation_rate_values, &crossover_rate_values)
}
