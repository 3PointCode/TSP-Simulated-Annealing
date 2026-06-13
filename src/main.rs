mod algorithms;
mod utils;
mod config;

use algorithms::{nearest_neighbor, simulated_annealing, NeighborMove};
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
    let mut best_sa_params: Option<(NeighborMove, f64, f64, f64, usize)> = None;
    let mut best_sa_route: Option<Vec<usize>> = None;

    for move_type in NeighborMove::ALL {
        for &initial_temp in &config.sa.initial_temps {
            for &min_temp in &config.sa.min_temps {
                for &cooling_rate in &config.sa.cooling_rates {
                    for &iterations_per_temp in &config.sa.iterations_per_temp_values {
                        println!("Running SA: Move: {}, T0: {}, Tmin: {}, Cooling: {}, Iterations: {}", move_type.as_str(), initial_temp, min_temp, cooling_rate, iterations_per_temp);

                        let start_time = Instant::now();
                        let result = simulated_annealing(&distance, &time, 
                            config.start_city, config.alpha, config.beta, initial_temp, min_temp, cooling_rate, iterations_per_temp, move_type);
                        let duration_ms = start_time.elapsed().as_millis();
                        
                        let ratio_to_best = result.best_cost / best_score;

                        println!("Optimized Cost: {:.2} | Time: {} ms | Ratio to best: {:.4}", result.best_cost, duration_ms, ratio_to_best);
                        append_result_row(results_path, move_type.as_str(), initial_temp, min_temp, cooling_rate, iterations_per_temp, cost, result.best_cost,
                            result.average_accepted_cost, result.accepted_moves, result.evaluated_candidates, best_score, duration_ms);

                        if result.best_cost < best_sa_cost {
                            best_sa_cost = result.best_cost;
                            best_sa_route = Some(result.best_route.clone());
                            best_sa_params = Some((
                                move_type,
                                initial_temp,
                                min_temp,
                                cooling_rate,
                                iterations_per_temp,
                            ));
                        }
                    }
                }
            }
        }
    }

    println!("\nBest SA Cost: {:.2}", best_sa_cost);

    if let Some((
        move_type,
        initial_temp,
        min_temp,
        cooling_rate,
        iterations_per_temp,
    )) = best_sa_params {
        println!("Best SA Params: Move: {}, T0={}, Tmin={}, Cooling={}, Iterations={}", move_type.as_str(), initial_temp, min_temp, cooling_rate, iterations_per_temp);
        println!("Best SA Ratio to best known: {:.6}", best_sa_cost / best_score);
        println!("Best SA Gap to best known: {:.3}%", ((best_sa_cost - best_score) / best_score) * 100.0);
    }

    if let Some(route) = &best_sa_route {
        println!("Best SA Route: {:?}", route);
    }

    println!("Results saved to: {}", results_path);
}