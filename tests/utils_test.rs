#[path = "../src/utils.rs"]
mod utils;
use utils::*;

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_generate_equally_spaced_values_custom_values() {
        // Test with custom parameters
        let min_val = 2.0;
        let max_val = 5.0;
        let length = 4;
        let expected_values = vec![2.0, 3.0, 4.0, 5.0];
        assert_eq!(
            generate_equally_spaced_values(min_val, max_val, length, false),
            expected_values
        );
    }

    #[test]
    fn test_generate_equally_spaced_values_edge_case_length_1() {
        // Test when length is 1
        let min_val = 2.0;
        let max_val = 5.0;
        let length = 1;
        let expected_values = vec![2.0];
        assert_eq!(
            generate_equally_spaced_values(min_val, max_val, length, false),
            expected_values
        );
    }

    #[test]
    fn test_generate_equally_spaced_values_edge_case_same_values() {
        // Test when min_val and max_val are the same
        let min_val = 3.0;
        let max_val = 3.0;
        let length = 5;
        let expected_values = vec![3.0, 3.0, 3.0, 3.0, 3.0];
        assert_eq!(
            generate_equally_spaced_values(min_val, max_val, length, false),
            expected_values
        );
    }

    #[test]
    fn test_generate_equally_spaced_values_invert_parameter() {
        let min_val = 2.0;
        let max_val = 5.0;
        let length = 4;
        let expected_values = vec![5.0, 4.0, 3.0, 2.0];
        assert_eq!(
            generate_equally_spaced_values(min_val, max_val, length, true),
            expected_values
        );
    }
    #[test]
    fn test_distribute_run_times_distribution_even() {
        let num_processes = 4;
        let run_times = 12;
        let expected_result = vec![4, 4, 4];
        assert_eq!(
            distribute_run_times(num_processes, run_times),
            expected_result
        );
    }

    #[test]
    fn test_distribute_run_times_distribution_remainder() {
        let num_processes = 3;
        let run_times = 10;
        let expected_result = vec![3, 3, 3, 1];
        assert_eq!(
            distribute_run_times(num_processes, run_times),
            expected_result
        );
    }

    #[test]
    fn test_distribute_run_times_distribution_single_process() {
        let num_processes = 1;
        let run_times = 10;
        let expected_result = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(
            distribute_run_times(num_processes, run_times),
            expected_result
        );
    }

    #[test]
    fn test_distribute_run_times_distribution_small_input() {
        let num_processes = 2;
        let run_times = 3;
        let expected_result = vec![2, 1];
        assert_eq!(
            distribute_run_times(num_processes, run_times),
            expected_result
        );
    }

    #[test]
    fn test_distribute_run_times_distribution_bigger_process_pool() {
        let num_processes = 10;
        let run_times = 3;
        let expected_result = vec![3];
        assert_eq!(
            distribute_run_times(num_processes, run_times),
            expected_result
        );
    }
}
