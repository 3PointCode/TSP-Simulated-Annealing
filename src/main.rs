mod algorithms;
mod utils;
mod config;

use algorithms::{nearest_neighbor, simulated_annealing};
use utils::{read_city_coords, build_distance_matrix, read_cost_pairs, find_best_pair, ensure_results_dir, init_results_file, append_result_row};
use config::Config;
use std::time::Instant;

fn main() {
    let config = Config::from_file("src/config.json");
    config.validate();

    let coords_a = read_city_coords(&config.data_a_path);
    let coords_b = read_city_coords(&config.data_b_path);    
    let distance = build_distance_matrix(&coords_a);
    let time = build_distance_matrix(&coords_b);

    let (_, cost) = nearest_neighbor(&distance, &time, config.start_city, config.alpha, config.beta);
    println!("Nearest Neighbor Cost: {:.2}", cost);

    let pairs = read_cost_pairs(&config.best_pairs_path);
    let (_, best_score) = find_best_pair(&pairs, config.alpha, config.beta).expect("No best pair found in dataset");
    println!("Best route cost from dataset: {:.2}", best_score);

    ensure_results_dir("results");
    let results_path = "results/sa_results.csv";
    init_results_file(results_path);

    let mut best_sa_cost = f64::INFINITY;
    let mut best_sa_params: Option<(f64, f64, f64, usize)> = None;

    for &initial_temp in &config.sa.initial_temps {
        for &min_temp in &config.sa.min_temps {
            for &cooling_rate in &config.sa.cooling_rates {
                for &iterations_per_temp in &config.sa.iterations_per_temp_values {
                    println!("Running SA: T0: {}, Tmin: {}, Cooling: {}, Iterations: {}", initial_temp, min_temp, cooling_rate, iterations_per_temp);

                    let start_time = Instant::now();
                    let (_, optimized_cost) = simulated_annealing(&distance, &time, 
                        config.start_city, config.alpha, config.beta, initial_temp, min_temp, cooling_rate, iterations_per_temp);
                    let duration_ms = start_time.elapsed().as_millis();

                    println!("Optimized Cost: {:.2} | Time: {} ms | Ratio to best: {:.4}", optimized_cost, duration_ms, optimized_cost / best_score);
                    append_result_row(results_path, initial_temp, min_temp, cooling_rate, iterations_per_temp, cost,
                        optimized_cost, best_score, duration_ms);

                    if optimized_cost < best_sa_cost {
                        best_sa_cost = optimized_cost;
                        best_sa_params = Some((initial_temp, min_temp, cooling_rate, iterations_per_temp));
                    }
                }
            }
        }
    }

    println!("\nBest SA Cost: {:.2}", best_sa_cost);

    if let Some((initial_temp, min_temp, cooling_rate, iterations_per_temp)) = best_sa_params {
        println!("Best SA Params: T0={}, Tmin={}, Cooling={}, Iterations={}",
            initial_temp, min_temp, cooling_rate, iterations_per_temp);
        println!("Best SA Ratio to best known: {:.4}", best_sa_cost / best_score);
    }
    println!("Results saved to: {}", results_path);
}