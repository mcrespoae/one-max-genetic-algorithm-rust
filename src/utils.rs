pub fn generate_equally_spaced_values(mut min_val: f64, mut max_val: f64, length: usize, invert: bool) -> Vec<f64> {
    if length <= 1 {
        return vec![min_val]; // Return a single value if length is 1 or less
    }

    if invert{
        let temp_max_val = min_val;
        min_val = max_val;
        max_val = temp_max_val;
    }


    let step = (max_val - min_val) / ((length - 1) as f64);
    let mut values: Vec<f64> = Vec::with_capacity(length);
    for i in 0..length {
        let value = min_val + i as f64 * step;
        values.push(value);
    }
    values
}


pub fn distribute_run_times(mut num_processes: usize, mut num_iterations: usize) -> Vec<usize> {
    if num_processes < 1 {
        num_processes = 1;
    }
    if num_iterations < 1 {
        num_iterations = 1;
    }

    let int_times = num_iterations / num_processes;
    let remainder_times = num_iterations % num_processes;

    let mut l = vec![num_processes; int_times];
    if remainder_times != 0 {
        l.push(remainder_times);
    }

    l
}