fn main() {
    let distance = vec![
        vec![0.0, 10.0, 15.0, 20.0],
        vec![10.0, 0.0, 35.0, 25.0],
        vec![15.0, 35.0, 0.0, 30.0],
        vec![20.0, 25.0, 30.0, 0.0],
    ];

    let time = vec![
        vec![0.0, 8.0, 12.0, 15.0],
        vec![8.0, 0.0, 20.0, 18.0],
        vec![12.0, 20.0, 0.0, 10.0],
        vec![15.0, 18.0, 10.0, 0.0],
    ];

    let (route, cost) = nearest_neighbor(&distance, &time, 0, 0.5, 0.5);
    println!("Route: {:?}", route);
    println!("Total cost: {}", cost);
}

fn travel_cost(i: usize, j: usize, distance: &Vec<Vec<f64>>, time: &Vec<Vec<f64>>, alpha: f64, beta: f64) -> f64 {
    alpha * distance[i][j] + beta * time[i][j]
}

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