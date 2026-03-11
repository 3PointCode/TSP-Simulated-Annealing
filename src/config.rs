use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub data_a_path: String,
    pub data_b_path: String,
    pub best_pairs_path: String,
    pub start_city: usize,
    pub alpha: f64,
    pub beta: f64,
    pub sa: SimulatedAnnealingConfig,
}

impl Config {
    pub fn validate(&self) {
        assert!(self.alpha >= 0.0, "α must be greater than 0");
        assert!(self.beta >= 0.0, "β must be greater than 0");
        assert!((self.alpha + self.beta - 1.0).abs() < 1e-12, "α + β must be equal to 1");
        assert!(!self.sa.initial_temps.is_empty(), "Initial temparature list must not be empty!");
        assert!(!self.sa.min_temps.is_empty(), "Minimal temperature list must not be empty!");
        assert!(!self.sa.cooling_rates.is_empty(), "Cooling rate list must not be empty!");
        assert!(!self.sa.iterations_per_temp_values.is_empty(), "Iterations per temperature list must not be empty!");

        for &temp in &self.sa.initial_temps {
            assert!(temp > 0.0, "Each Initial Temperature value must be greater than 0");
        }

        for &temp in &self.sa.min_temps {
            assert!(temp > 0.0, "Each Minimal Temperature value must be greater than 0");
        }

        for &temp in &self.sa.cooling_rates {
            assert!(temp > 0.0 && temp < 1.0, "Each Cooling Rate value must be in (0, 1)");
        }

        for &temp in &self.sa.iterations_per_temp_values {
            assert!(temp > 0, "Each Iterations Per Temperature value must be greater than 0");
        }

        for &init in &self.sa.initial_temps {
            for &min in &self.sa.min_temps {
                assert!(init > min, "Each Initial Temperature value must be bigger than Minimal Temperature value!");
            }
        }
    }

    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read the config file!");
        let config: Config = serde_json::from_str(&content).expect("Failed to parse the config.json file!");

        config
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimulatedAnnealingConfig {
    pub initial_temps: Vec<f64>,
    pub min_temps: Vec<f64>,
    pub cooling_rates: Vec<f64>,
    pub iterations_per_temp_values: Vec<usize>,
}