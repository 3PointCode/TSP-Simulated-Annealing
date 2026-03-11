mod algorithms;
mod utils;
mod config;

use algorithms::{nearest_neighbor, simmulated_annealing};
use utils::{read_city_coords, build_distance_matrix, read_cost_pairs, find_best_pair};
use config::Config;

fn main() {
    let config = Config::from_file("src/config.json");
    config.validate();

    let coords_a = read_city_coords(&config.data_a_path);
    let coords_b = read_city_coords(&config.data_b_path);    
    let distance = build_distance_matrix(&coords_a);
    let time = build_distance_matrix(&coords_b);

    let (_, cost) = nearest_neighbor(&distance, &time, config.start_city, config.alpha, config.beta);
    println!("Nearest Neighbor Cost: {:.2}", cost);

    for &initial_temp in &config.sa.initial_temps {
        for &min_temp in &config.sa.min_temps {
            for &cooling_rate in &config.sa.cooling_rates {
                for &iterations_per_temp in &config.sa.iterations_per_temp_values {
                    println!("Running SA: T0: {}, Tmin: {}, Cooling: {}, Iterations: {}", initial_temp, min_temp, cooling_rate, iterations_per_temp);

                    let (_, optimized_cost) = simmulated_annealing(&distance, &time, 
                        config.start_city, config.alpha, config.beta, initial_temp, min_temp, cooling_rate, iterations_per_temp);
                    
                    println!("Optimized Cost: {:.2}", optimized_cost);
                }
            }
        }
    }

    let pairs = read_cost_pairs(&config.best_pairs_path);
    if let Some((best_pair, best_score)) = find_best_pair(&pairs, config.alpha, config.beta) {
        println!("Best Route Pair from dataset: {:?}", best_pair);
        println!("Best Route Cost: {}", best_score);
    }
}