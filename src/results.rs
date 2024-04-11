#[derive(Clone)]
pub struct Results {
    pub max_generations: u32,
    pub max_fitness: f64,

    pub generations: Vec<u32>,
    pub generation_fitnesses: Vec<f64>,
    pub best_fitnesses: Vec<f64>,
    pub score: f64,
    pub total_generations: usize,
    pub avg_generation: f64,
    pub avg_generation_fitness: f64,
    pub avg_best_fitness: f64,
    pub best_fitness: f64
}

impl Results {
    pub fn new(max_generations: u32, max_fitness: f64) -> Self{
        Results{
            max_generations,
            max_fitness,
            generations: Vec::new(),
            generation_fitnesses: Vec::new(),
            best_fitnesses: Vec::new(),
            score: 0.0,
            total_generations: 0,
            avg_generation: 0.0,
            avg_generation_fitness: 0.0,
            avg_best_fitness: 0.0,
            best_fitness: 0.0,
        }
    }

    pub fn add_result(&mut self, generation: u32, generation_fitness: f64, best_fitness: f64) {
        self.generations.push(generation);
        self.generation_fitnesses.push(generation_fitness);
        self.best_fitnesses.push(best_fitness);
        self.set_overall_values();
    }

    pub fn set_overall_values(&mut self) {
        self.total_generations = self.generations.len() as usize;

        self.avg_generation = if self.total_generations > 0 {
            self.generations.iter().sum::<u32>() as f64 / self.total_generations as f64
        } else {
            self.max_generations as f64
        };
        self.avg_generation_fitness = if self.total_generations > 0 {
            self.generation_fitnesses.iter().sum::<f64>() / self.total_generations as f64
        } else {
            0.0
        };
        self.avg_best_fitness = if self.total_generations > 0 {
            self.best_fitnesses.iter().sum::<f64>() / self.total_generations as f64
        } else {
            0.0
        };
        self.best_fitness = *self.best_fitnesses.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&0.0);
        self.calculate_ponderate_score();
    }

    pub fn calculate_ponderate_score(&mut self) {
        if self.generations.is_empty() {
            self.score = 0.0;
            return;
        }

        // Define weights for each metric
        let weight_best_fitness: f64 = 0.4;
        let weight_avg_generation_fitness: f64 = 0.3;
        let weight_avg_best_fitness: f64 = 0.2;
        let weight_avg_generation: f64 = 0.1;

        // Adjusting the scores based on criteria
        let score_best_fitness: f64 = self.best_fitness / self.max_fitness;
        let score_avg_generation: f64 = 1.0 - (self.avg_generation - 1.0) / self.max_generations as f64;  // Inverting the avg_generation score and scaling it to 0-1 range
        let score_avg_generation_fitness: f64 = self.avg_generation_fitness;  // avg_generation_fitness already in desired range
        let score_avg_best_fitness: f64 = self.avg_best_fitness / self.max_fitness;  // Scaling avg_best_fitness to 0-1 range

        // Calculate the ponderate score
        self.score = weight_best_fitness * score_best_fitness +
                     weight_avg_generation * score_avg_generation +
                     weight_avg_generation_fitness * score_avg_generation_fitness +
                     weight_avg_best_fitness * score_avg_best_fitness;
    }

    pub fn get_score(&self) -> f64 {
        self.score
    }

    pub fn print(&self) {
        println!("Overall Score:          {:.3}", self.score);
        println!("Best Fitness:           {}", self.best_fitness);
        println!("Avg Generation Fitness: {:.3}", self.avg_generation_fitness);
        println!("Avg Best Fitness:       {:.3}", self.avg_best_fitness);
        println!("Avg Generations Run:    {:.3} of {}", self.avg_generation, self.max_generations);
    }

}


