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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn valid_config() -> Config {
        Config {
            data_a_path: "a.tsp".into(),
            data_b_path: "b.tsp".into(),
            best_pairs_path: "best.txt".into(),
            start_city: 0,
            alpha: 0.5,
            beta: 0.5,
            sa: SimulatedAnnealingConfig {
                initial_temps: vec![1000.0],
                min_temps: vec![0.1],
                cooling_rates: vec![0.9],
                iterations_per_temp_values: vec![100],
            },
        }
    }

    #[test]
    fn test_validate_valid_config() {
        valid_config().validate();
    }

    #[test]
    #[should_panic]
    fn test_validate_alpha_beta_sum_not_one() {
        let mut c = valid_config();
        c.alpha = 0.3;
        c.beta = 0.3;
        c.validate();
    }

    #[test]
    #[should_panic]
    fn test_validate_negative_alpha() {
        let mut c = valid_config();
        c.alpha = -0.5;
        c.beta = 1.5;
        c.validate();
    }

    #[test]
    #[should_panic]
    fn test_validate_empty_initial_temps() {
        let mut c = valid_config();
        c.sa.initial_temps = vec![];
        c.validate();
    }

    #[test]
    #[should_panic]
    fn test_validate_empty_min_temps() {
        let mut c = valid_config();
        c.sa.min_temps = vec![];
        c.validate();
    }

    #[test]
    #[should_panic]
    fn test_validate_empty_cooling_rates() {
        let mut c = valid_config();
        c.sa.cooling_rates = vec![];
        c.validate();
    }

    #[test]
    #[should_panic]
    fn test_validate_cooling_rate_out_of_range() {
        let mut c = valid_config();
        c.sa.cooling_rates = vec![1.5];
        c.validate();
    }

    #[test]
    #[should_panic]
    fn test_validate_initial_temp_less_than_min_temp() {
        let mut c = valid_config();
        c.sa.initial_temps = vec![0.05];
        c.sa.min_temps = vec![0.1];
        c.validate();
    }

    #[test]
    fn test_from_file_valid_json() {
        let json = r#"{
            "data_a_path": "a.tsp",
            "data_b_path": "b.tsp",
            "best_pairs_path": "best.txt",
            "start_city": 0,
            "alpha": 0.5,
            "beta": 0.5,
            "sa": {
                "initial_temps": [1000.0],
                "min_temps": [0.1],
                "cooling_rates": [0.9],
                "iterations_per_temp_values": [100]
            }
        }"#;
        let mut f = tempfile::NamedTempFile::new().unwrap();
        write!(f, "{}", json).unwrap();
        let config = Config::from_file(f.path().to_str().unwrap());
        assert!((config.alpha - 0.5).abs() < 1e-9);
        assert_eq!(config.start_city, 0);
        assert_eq!(config.sa.initial_temps, vec![1000.0]);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimulatedAnnealingConfig {
    pub initial_temps: Vec<f64>,
    pub min_temps: Vec<f64>,
    pub cooling_rates: Vec<f64>,
    pub iterations_per_temp_values: Vec<usize>,
}