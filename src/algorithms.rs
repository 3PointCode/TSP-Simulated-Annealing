use rand::Rng;

// Function to calculate the travel cost between two cities based on distance and time
// The cost is calculated as a weighted sum of distance and time, where alpha and beta are the weights for distance and time respectively
fn travel_cost(i: usize, j: usize, distance: &[Vec<f64>], time: &[Vec<f64>], alpha: f64, beta: f64) -> f64 {
    alpha * distance[i][j] + beta * time[i][j]
}

fn route_cost(route: &[usize], distance: &[Vec<f64>], time: &[Vec<f64>], alpha: f64, beta: f64) -> f64 {
    let mut total_cost = 0.0;

    for i in 0..route.len() - 1 {
        total_cost += travel_cost(route[i], route[i + 1], distance, time, alpha, beta)
    }

    total_cost
}

// Nearest Neighbor heuristic for the Traveling Salesman Problem used to find an initial solution before simulated annealing optimization
pub fn nearest_neighbor(distance: &[Vec<f64>], time: &[Vec<f64>], start: usize, alpha: f64, beta: f64) -> (Vec<usize>, f64) {
    let cities_number: usize = distance.len();
    let mut visited_cities: Vec<bool> = vec![false; cities_number];
    let mut route: Vec<usize> = Vec::new();
    let mut total_cost = 0.0;
    let mut current_city = start;

    visited_cities[current_city] = true;
    route.push(current_city);

    while route.len() < cities_number {
        let mut nearest_city: Option<usize> = None;
        let mut nearest_cost = f64::INFINITY;
        
        for j in 0..cities_number {
            if !visited_cities[j] {
                let current_cost = travel_cost(current_city, j, distance, time, alpha, beta);
                
                if current_cost < nearest_cost {
                    nearest_cost = current_cost;
                    nearest_city = Some(j);
                }
            }
        }

        let next_city = nearest_city.expect("City not found");

        visited_cities[next_city] = true;
        route.push(next_city);
        total_cost += nearest_cost;

        current_city = next_city;
    }

    total_cost += travel_cost(current_city, start, distance, time, alpha, beta);
    route.push(start);

    (route, total_cost)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeighborMove {
    TwoOpt,
    Swap,
    Insert,
}

impl NeighborMove {
    pub const ALL: [NeighborMove; 3] = [
        NeighborMove::TwoOpt,
        NeighborMove::Swap,
        NeighborMove::Insert
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            NeighborMove::TwoOpt => "two_opt",
            NeighborMove::Swap => "swap",
            NeighborMove::Insert => "insert",
        }
    }
}

// Function to generate a new route during the simulated annealing algorithm using the 2-opt
fn generate_neighbor<R: Rng + ?Sized>(route: &[usize], rng: &mut R, move_type: NeighborMove) -> Vec<usize> {
    let mut neighbor = route.to_vec();

    if neighbor.len() <= 3 {
        return neighbor;
    }

    let first_internal_index = 1;
    let last_internal_exclusive = route.len() - 1;

    match move_type {
        NeighborMove::TwoOpt => {
            // i < j, reverse the path between these two points
            let i = rng.gen_range(first_internal_index..last_internal_exclusive - 1);
            let j = rng.gen_range(i + 1..last_internal_exclusive);
            neighbor[i..=j].reverse();
        }

        NeighborMove::Swap => {
            let i = rng.gen_range(first_internal_index..last_internal_exclusive);
            let mut j = rng.gen_range(first_internal_index..last_internal_exclusive);

            while i == j {
                j = rng.gen_range(first_internal_index..last_internal_exclusive);
            }

            neighbor.swap(i, j);
        }

        NeighborMove::Insert => {
            let i = rng.gen_range(first_internal_index..last_internal_exclusive);
            let city = neighbor.remove(i);
            // insert the removed element
            let mut j = rng.gen_range(first_internal_index..neighbor.len());

            // don't insert the element in the same place
            while i == j {
                j = rng.gen_range(first_internal_index..neighbor.len());
            }

            neighbor.insert(j, city);
        }
    }

    neighbor
}

#[derive(Debug, Clone)]
pub struct SimulatedAnnealingResult {
    pub best_route: Vec<usize>,
    pub best_cost: f64,
    pub average_accepted_cost: Option<f64>,
    pub accepted_moves: usize,
    pub evaluated_candidates: usize,
}

pub fn simulated_annealing(distance: &[Vec<f64>], time: &[Vec<f64>], start: usize, alpha: f64, beta: f64,
    initial_temp: f64, min_temp: f64, cooling_rate: f64, iterations_per_temp: usize, move_type: NeighborMove) -> SimulatedAnnealingResult {
        let mut rng = rand::thread_rng();

        let (initial_route, initial_cost) = nearest_neighbor(distance, time, start, alpha, beta);
        let mut current_route = initial_route;
        let mut current_cost = initial_cost;

        let mut best_route = current_route.clone();
        let mut best_cost = current_cost;

        let mut accepted_cost_sum = 0.0;
        let mut accepted_moves = 0usize;
        let mut evaluated_candidates = 0usize;

        let mut temperature = initial_temp;

        while temperature > min_temp {
            for _ in 0..iterations_per_temp {
                let candidate_route = generate_neighbor(&current_route, &mut rng, move_type);
                let candidate_cost = route_cost(&candidate_route, distance, time, alpha, beta);

                evaluated_candidates += 1;
                let cost_delta = candidate_cost - current_cost;

                let accepted = if cost_delta < 0.0 {
                    true
                } else {
                    let acceptance_probability = (-cost_delta / temperature).exp();
                    let random_value: f64 = rng.r#gen();
                    random_value < acceptance_probability
                };

                if accepted {
                    current_route = candidate_route;
                    current_cost = candidate_cost;
                    
                    // add only accepted scores to the sum
                    accepted_cost_sum += current_cost;
                    accepted_moves += 1;

                    // update the best current solution found
                    if current_cost < best_cost {
                        best_route = current_route.clone();
                        best_cost = current_cost;
                    }
                }
            }

            temperature *= cooling_rate;
        }
        
        let average_accepted_cost = if accepted_moves > 0 {
            Some(accepted_cost_sum / accepted_moves as f64)
        } else {
            None
        };

        SimulatedAnnealingResult {
            best_route,
            best_cost,
            average_accepted_cost,
            accepted_moves,
            evaluated_candidates,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_matrices() -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let d = vec![
            vec![0.0, 10.0, 20.0],
            vec![10.0, 0.0, 10.0],
            vec![20.0, 10.0, 0.0],
        ];
        let t = vec![
            vec![0.0, 4.0, 8.0],
            vec![4.0, 0.0, 4.0],
            vec![8.0, 4.0, 0.0],
        ];
        (d, t)
    }

    #[test]
    fn test_travel_cost() {
        let (d, t) = simple_matrices();
        // 0.5 * 10 + 0.5 * 4 = 7.0
        let cost = travel_cost(0, 1, &d, &t, 0.5, 0.5);
        assert!((cost - 7.0).abs() < 1e-9);
    }

    #[test]
    fn test_travel_cost_only_distance() {
        let (d, t) = simple_matrices();
        let cost = travel_cost(0, 2, &d, &t, 1.0, 0.0);
        assert!((cost - 20.0).abs() < 1e-9);
    }

    #[test]
    fn test_route_cost_two_edges() {
        let (d, t) = simple_matrices();
        // 0→1: 7.0, 1→2: 7.0 => łącznie 14.0
        let cost = route_cost(&[0, 1, 2], &d, &t, 0.5, 0.5);
        assert!((cost - 14.0).abs() < 1e-9);
    }

    #[test]
    fn test_route_cost_single_edge() {
        let (d, t) = simple_matrices();
        let cost = route_cost(&[0, 2], &d, &t, 0.5, 0.5);
        // 0.5*20 + 0.5*8 = 14.0
        assert!((cost - 14.0).abs() < 1e-9);
    }

    #[test]
    fn test_nearest_neighbour_7_cities() {
        let distance = vec![
            vec![0.0, 12.0, 10.0, 19.0, 8.0, 14.0, 16.0],
            vec![12.0, 0.0, 3.0, 7.0, 2.0, 8.0, 9.0],
            vec![10.0, 3.0, 0.0, 6.0, 20.0, 4.0, 5.0],
            vec![19.0, 7.0, 6.0, 0.0, 4.0, 3.0, 2.0],
            vec![8.0, 2.0, 20.0, 4.0, 0.0, 6.0, 7.0],
            vec![14.0, 8.0, 4.0, 3.0, 6.0, 0.0, 1.0],
            vec![16.0, 9.0, 5.0, 2.0, 7.0, 1.0, 0.0],
        ];

        let time = vec![
            vec![0.0, 10.0, 8.0, 15.0, 7.0, 11.0, 13.0],
            vec![10.0, 0.0, 2.0, 6.0, 3.0, 7.0, 8.0],
            vec![8.0, 2.0, 0.0, 5.0, 17.0, 3.0, 4.0],
            vec![15.0, 6.0, 5.0, 0.0, 3.0, 2.0, 1.0],
            vec![7.0, 3.0, 17.0, 3.0, 0.0, 5.0, 6.0],
            vec![11.0, 7.0, 3.0, 2.0, 5.0, 0.0, 1.0],
            vec![13.0, 8.0, 4.0, 1.0, 6.0, 1.0, 0.0],
        ];

        let (route, cost) = nearest_neighbor(&distance, &time, 0, 0.5, 0.5);
        let expected_cost = 35.5;
        let expected_route = vec![0, 4, 1, 2, 5, 6, 3, 0];
        assert!((cost - expected_cost).abs() < 1e-6);
        assert_eq!(route, expected_route);
    }

    #[test]
    fn test_nearest_neighbor_starts_and_ends_at_start() {
        let (d, t) = simple_matrices();
        let (route, _) = nearest_neighbor(&d, &t, 0, 0.5, 0.5);
        assert_eq!(route[0], 0);
        assert_eq!(*route.last().unwrap(), 0);
    }

    #[test]
    fn test_nearest_neighbor_visits_all_cities() {
        let (d, t) = simple_matrices();
        let (route, _) = nearest_neighbor(&d, &t, 0, 0.5, 0.5);
        let mut cities: Vec<usize> = route[..route.len() - 1].to_vec();
        cities.sort();
        assert_eq!(cities, vec![0, 1, 2]);
    }

    #[test]
    fn test_generate_neighbor_preserves_cities() {
        let route = vec![0, 1, 2, 3, 4, 5, 0];
        let mut rng = rand::thread_rng();
        let neighbor = generate_neighbor(&route, &mut rng, NeighborMove::TwoOpt);
        let mut r_sorted = route.clone();
        let mut n_sorted = neighbor.clone();
        r_sorted.sort();
        n_sorted.sort();
        assert_eq!(r_sorted, n_sorted);
    }

    #[test]
    fn test_generate_neighbor_short_route_unchanged() {
        let route = vec![0, 1, 2, 0]; // długość 4 → bez zmian
        let mut rng = rand::thread_rng();
        let neighbor = generate_neighbor(&route, &mut rng, NeighborMove::TwoOpt);
        assert_eq!(route, neighbor);
    }

    #[test]
    fn test_simulated_annealing_visits_all_cities() {
        let (d, t) = simple_matrices();
        let result = simulated_annealing(&d, &t, 0, 0.5, 0.5, 100.0, 0.1, 0.9, 10, NeighborMove::TwoOpt);
        let mut cities: Vec<usize> = result.best_route[..result.best_route.len() - 1].to_vec();
        cities.sort();
        assert_eq!(cities, vec![0, 1, 2]);
    }

    #[test]
    fn test_simulated_annealing_starts_and_ends_at_start() {
        let (d, t) = simple_matrices();
        let result = simulated_annealing(&d, &t, 0, 0.5, 0.5, 100.0, 0.1, 0.9, 10, NeighborMove::TwoOpt);
        assert_eq!(result.best_route.first().copied(), Some(0));
        assert_eq!(result.best_route.last().copied(), Some(0));
    }

    #[test]
    fn test_simulated_annealing_never_returns_cost_worse_than_initial() {
        let (distance, time) = simple_matrices();
        let (_, initial_cost) = nearest_neighbor(&distance, &time, 0, 0.5, 0.5);
        let result = simulated_annealing(&distance, &time, 0, 0.5, 0.5, 100.0, 0.1, 0.9, 10, NeighborMove::TwoOpt);

        assert!(result.best_cost <= initial_cost + 1e-9);
    }
}