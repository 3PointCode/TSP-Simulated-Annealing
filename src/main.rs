use std::fs;

fn main() {
    let coords = read_city_cords("data/euclidA100.tsp");
    let distance = build_distance_matrix(&coords);
    let coords = read_city_cords("data/euclidB100.tsp");
    let time = build_distance_matrix(&coords);
    let (route, cost) = nearest_neighbor(&distance, &time, 0, 0.5, 0.5);
    println!("Route: {:?}", route);
    println!("Total cost: {}", cost);
}

// Function to calculate the travel cost between two cities based on distance and time
// The cost is calculated as a weighted sum of distance and time, where alpha and beta are the weights for distance and time respectively
fn travel_cost(i: usize, j: usize, distance: &Vec<Vec<f64>>, time: &Vec<Vec<f64>>, alpha: f64, beta: f64) -> f64 {
    alpha * distance[i][j] + beta * time[i][j]
}

// Nearest Neighbor heuristic for the Traveling Salesman Problem used to find an initial solution before simulated annealing optimization
fn nearest_neighbor(distance: &Vec<Vec<f64>>, time: &Vec<Vec<f64>>, start: usize, alpha: f64, beta: f64) -> (Vec<usize>, f64) {
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

// Function to read city coordinates from a file and return them as a vector of tuples (x, y)
fn read_city_cords(path: &str) -> Vec<(f64,f64)> {
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