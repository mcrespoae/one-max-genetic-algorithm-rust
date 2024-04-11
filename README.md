
# Genetic Algorithm: Solving the One Max Problem in Rust

This project implements a genetic algorithm to solve the One Max Problem, which involves finding a binary string of maximum length where all bits are set to 1.

## Requirements
- [Rust](https://www.rust-lang.org/tools/install).
- [Make](https://www.gnu.org/software/make/) *(Optional)* . For Windows users, consider installing Make via [Chocolatey](https://chocolatey.org/install) using  `choco install make `


## Installation

To set up this project, begin by cloning the repository and navigating to the project directory. Install the dependencies and compile the project by running:
```bash
cargo build
```
Alternatively, you can execute it using Make:
```bash
make build-preview
```
Or *(with the release flag)*:
```bash
make build
```

## Project tree
- `Makefile`: Contains useful project related commands.
- `src`
    - `main.rs`: Contains the main entry point for running the algorithm.
    - `one_max_genetic_algorithm.rs`: Implementation of the genetic algorithm.
    - `results.rs`: Contains a class for storing and computing the results of the genetic algorithm.
    - `utils.rs`: Contains helper functions.
- `tests`
    - `test_one_max_genetic_algorithm.rs`: Unittests for the genetic algorithm.
    - `test_results.rs`: Unittests for the Results class.
    - `test_utils.rs`: Unittests for the utils file.

## External Dependencies
- `tqdm`: Used for displaying progress bars during execution.
- `rand`: Provides random number generation.
- `num_cpus`: Provides the count of CPUs on the current machine to adjust the number of threads in concurrency.
- `assert_approx_eq`: Useful for unittesting.

## Usage
To run the genetic algorithm, execute:
```bash
cargo run
```
You can adjust various parameters in `main.rs` to customize the genetic algorithm's behavior. These parameters include:

- `RUN_TIMES`: Number of times to run the genetic algorithm.
- `GENERATIONS`: Number of generations in the genetic algorithm.
- `POPULATION_SIZE`: Size of the population in each generation.
- `GENOME_LENGTH`: Length of the binary string.
- `TARGET_GENERATION_FITNESS`: Target fitness for a generation to be considered successful and skip the next iterations. From 0 to 1. Values close to 1.0 will yield better results.
- `TARGET_PROBLEM_FITNESS`: Target fitness for the whole problem to be marked as solved. From 0 to 1. Values very close to 1.0 will not stop the execution.
- `MUTATION_RATE_MIN`: Minimum mutation rate.
- `MUTATION_RATE_MAX`: Maximum mutation rate.
- `CROSSOVER_RATE_MIN`: Minimum crossover rate.
- `CROSSOVER_RATE_MAX`: Maximum crossover rate.

If you're using Make, you can also execute the main file using the following command:

```bash
make run
```
This command will generate a build with the release flag and run it. To run it without the release flag, you can:
```bash
make run-preview
```

## Algorithm Overview
The genetic algorithm proceeds as follows:

1. **Initialization**: Initialize a population of binary strings randomly.
2. **Evaluation**: Evaluate the fitness of each individual in the population.
3. **Selection**: Select individuals for reproduction based on their fitness using either roulette or tournament selection.
4. **Crossover**: Produce offspring by combining genetic material from selected individuals.
5. **Mutation**: Introduce random changes to the offspring's genetic material.
6. **Replacement**: Replace the old generation with the new generation.
7. **Termination**: Repeat steps 2-6 until a termination condition is met, such as reaching a maximum number of generations or achieving a target fitness level.

## Results
The project includes functionality for processing the genetic algorithm with different mutation rates and crossover rates to determine the optimal combination for solving the One Max Problem efficiently; the results are displayed, showing the best mutation rate and crossover rate found during the processing.

## Optimization
The genetic algorithm employs concurrent processing techniques for parallel execution, enhancing runtime performance.

## Testing
The code is equipped with comprehensive unit tests to ensure reliability.

To execute them, use the command:
```bash
cargo test
```
Or, they can be executed using the Make command.
```bash
make test
```

## Contributors

- [Mario Crespo](https://github.com/mcrespoae)

