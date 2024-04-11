#[path = "../src/results.rs"]
mod results;
use results::*;
use assert_approx_eq::assert_approx_eq;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_result() {
        let mut results = Results::new(100, 1.0);
        results.add_result(60, 0.8, 0.9);

        assert_eq!(results.generations.len(), 1);
        assert_eq!(results.generation_fitnesses.len(), 1);
        assert_eq!(results.best_fitnesses.len(), 1);
    }

    #[test]
    fn test_set_overall_values() {
        let mut results = Results::new(100, 1.0);
        results.add_result(1, 0.8, 0.9);
        results.add_result(2, 0.85, 0.92);

        assert_eq!(results.total_generations, 2);
        assert_approx_eq!(results.avg_generation, 1.5);
        assert_approx_eq!(results.avg_generation_fitness, 0.825);
        assert_approx_eq!(results.avg_best_fitness, 0.91);
        assert_approx_eq!(results.best_fitness, 0.92);
    }

    #[test]
    fn test_calculate_ponderate_score() {
        let mut results = Results::new(100, 1.0);
        results.add_result(1, 0.8, 0.9);
        results.add_result(2, 0.85, 0.92);

        let expected_score =
            0.4 * (0.92 / 1.0) +
            0.1 * (1.0 - (1.5 - 1.0) / 100.0) +
            0.3 * 0.825 +
            0.2 * (0.91 / 1.0);

        assert_approx_eq!(results.get_score(), expected_score);
    }
}