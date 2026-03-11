use std::fs;
use rand::Rng;

fn main() {
    let pairs = read_cost_pairs("data/best.euclidAB300.tsp");
    let coords = read_city_coords("data/euclidA300.tsp");
    let distance = build_distance_matrix(&coords);
    let coords = read_city_coords("data/euclidB300.tsp");
    let time = build_distance_matrix(&coords);
    let (_, cost) = nearest_neighbor(&distance, &time, 0, 0.5, 0.5);
    println!("Total cost: {}", cost.round());

    let (optimized_route, optimized_cost) = simmulated_annealing(&distance, &time, 0, 0.5, 0.5, 10000.0, 0.001, 0.995, 1000);
    match find_best_pair(&pairs, 0.5, 0.5) {
        Some(((a, b), score)) => {
            println!("Best Route Pair: ({}, {})", a, b);
            println!("Best Route Score: {}", score);
        },
        None => println!("No pairs found!"),
    }
    println!("Optimized Route: {:?}", optimized_route);
    println!("Optimized Cost: {}", optimized_cost.round());
}

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
fn nearest_neighbor(distance: &[Vec<f64>], time: &[Vec<f64>], start: usize, alpha: f64, beta: f64) -> (Vec<usize>, f64) {
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

// Function to generate a new route during the simulated annealing algorithm using the 2-opt
fn generate_neighbor(route: &[usize], rng: &mut impl Rng) -> Vec<usize> {
    let mut neighbor = route.to_vec();

    if neighbor.len() <= 4 {
        return neighbor;
    }

    let n = neighbor.len();
    let mut i = rng.gen_range(1..n - 2);
    let mut j = rng.gen_range(i + 1..n - 1);

    if i > j {
        std::mem::swap(&mut i, &mut j);
    }

    // reversing the route between i and j, example for i=1, j=4: [0, 1, 2, 3, 4, 5] -> [0, 4, 3, 2, 1, 5]
    neighbor[i..=j].reverse();
    neighbor
}

fn simmulated_annealing(distance: &[Vec<f64>], time: &[Vec<f64>], start: usize, alpha: f64, beta: f64,
    initial_temp: f64, min_temp: f64, cooling_rate: f64, iterations_per_temp: usize) -> (Vec<usize>, f64) {
        let mut rng = rand::thread_rng();

        let (initial_route, initial_cost) = nearest_neighbor(distance, time, start, alpha, beta);
        let mut current_route = initial_route;
        let mut current_cost = initial_cost;

        let mut best_route = current_route.clone();
        let mut best_cost = current_cost;

        let mut temperature = initial_temp;

        while temperature > min_temp {
            for _ in 0..iterations_per_temp {
                let candidate_route = generate_neighbor(&current_route,&mut rng);
                let candidate_cost = route_cost(&candidate_route, distance, time, alpha, beta);

                let cost_delta = candidate_cost - current_cost;

                if cost_delta < 0.0 {
                    current_route = candidate_route;
                    current_cost = candidate_cost;
                } else {
                    let acceptance_probability = (-cost_delta / temperature).exp();
                    let random_value: f64 = rng.r#gen();
                    
                    if random_value < acceptance_probability {
                        current_route = candidate_route;
                        current_cost = candidate_cost;
                    }
                }

                if current_cost < best_cost {
                    best_route = current_route.clone();
                    best_cost = current_cost;
                }
            }

            temperature *= cooling_rate;
        }
        
        (best_route, best_cost)
}

// Function to read city coordinates from a file and return them as a vector of tuples (x, y)
fn read_city_coords(path: &str) -> Vec<(f64,f64)> {
    let content = fs::read_to_string(path).unwrap();
    let mut coords: Vec<(f64, f64)> = Vec::new();
    let mut reading: bool = false;

    for line in content.lines() {
        if line.starts_with("NODE_COORD_SECTION") {
            reading = true;
            continue;
        }

        if reading {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 3 {
                let x: f64 = parts[1].parse().unwrap();
                let y: f64 = parts[2].parse().unwrap();
                coords.push((x, y));
            }
        }
    }

    coords
}

fn read_cost_pairs(path: &str) -> Vec<(f64, f64)> {
    let content = fs::read_to_string(path).expect("Failed to read the file!");
    let mut pairs = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let a: f64 = parts[0].parse().expect("Couldn't parse the value!");
            let b: f64 = parts[1].parse().expect("Couldn't parse the value!");
            
            pairs.push((a, b));
        }
    }

    pairs
}

fn weighted_score(a: f64, b: f64, alpha: f64, beta: f64) -> f64 {
    alpha * a + beta * b
}

fn find_best_pair(pairs: &[(f64, f64)], alpha: f64, beta: f64) -> Option<((f64, f64), f64)> {
    if pairs.is_empty() {
        return None;
    }

    let mut best_pair = pairs[0];
    let mut best_score = weighted_score(pairs[0].0, pairs[0].1, alpha, beta);

    for &(a, b) in &pairs[1..] {
        let score = weighted_score(a, b, alpha, beta);

        if score < best_score {
            best_pair = (a, b);
            best_score = score;
        }
    }

    Some((best_pair, best_score))
}

// Function to build a distance matrix from city coordinates using Euclidean distance
fn build_distance_matrix(coords: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
    let city_num = coords.len();
    let mut matrix = vec![vec![0.0; city_num]; city_num];

    for i in 0..city_num {
        for j in 0..city_num {
            let dx = coords[i].0 - coords[j].0;
            let dy = coords[i].1 - coords[j].1;

            matrix[i][j] = (dx*dx + dy*dy).sqrt();
        }
    }
    
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
}