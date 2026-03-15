use std::fs;

// Function to read city coordinates from a file and return them as a vector of tuples (x, y)
pub fn read_city_coords(path: &str) -> Vec<(f64,f64)> {
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
pub fn build_distance_matrix(coords: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
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

pub fn read_cost_pairs(path: &str) -> Vec<(f64, f64)> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_build_distance_matrix_diagonal_zero() {
        let coords = vec![(0.0, 0.0), (3.0, 4.0)];
        let m = build_distance_matrix(&coords);
        assert_eq!(m[0][0], 0.0);
        assert_eq!(m[1][1], 0.0);
    }

    #[test]
    fn test_build_distance_matrix_known_distance() {
        // trójkąt pitagorejski 3-4-5
        let coords = vec![(0.0, 0.0), (3.0, 4.0)];
        let m = build_distance_matrix(&coords);
        assert!((m[0][1] - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_build_distance_matrix_symmetry() {
        let coords = vec![(0.0, 0.0), (3.0, 4.0), (6.0, 0.0)];
        let m = build_distance_matrix(&coords);
        for i in 0..3 {
            for j in 0..3 {
                assert!((m[i][j] - m[j][i]).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn test_find_best_pair_empty() {
        assert_eq!(find_best_pair(&[], 0.5, 0.5), None);
    }

    #[test]
    fn test_find_best_pair_single() {
        let result = find_best_pair(&[(3.0, 5.0)], 0.5, 0.5).unwrap();
        assert_eq!(result.0, (3.0, 5.0));
        assert!((result.1 - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_find_best_pair_picks_minimum() {
        let pairs = vec![(10.0, 10.0), (2.0, 2.0), (5.0, 5.0)];
        let (best, score) = find_best_pair(&pairs, 0.5, 0.5).unwrap();
        assert_eq!(best, (2.0, 2.0));
        assert!((score - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_find_best_pair_respects_weights() {
        // (1.0, 10.0) vs (10.0, 1.0) — przy alpha=0.9 wygrywa ta z mniejszą odległością
        let pairs = vec![(1.0, 10.0), (10.0, 1.0)];
        let (best, _) = find_best_pair(&pairs, 0.9, 0.1).unwrap();
        assert_eq!(best, (1.0, 10.0));
    }

    #[test]
    fn test_read_city_coords() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "SOME_HEADER").unwrap();
        writeln!(f, "NODE_COORD_SECTION").unwrap();
        writeln!(f, "1 0.0 0.0").unwrap();
        writeln!(f, "2 3.0 4.0").unwrap();
        let coords = read_city_coords(f.path().to_str().unwrap());
        assert_eq!(coords, vec![(0.0, 0.0), (3.0, 4.0)]);
    }

    #[test]
    fn test_read_city_coords_empty_section() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "NODE_COORD_SECTION").unwrap();
        let coords = read_city_coords(f.path().to_str().unwrap());
        assert!(coords.is_empty());
    }

    #[test]
    fn test_read_cost_pairs() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "1.5 2.5").unwrap();
        writeln!(f, "3.0 4.0").unwrap();
        let pairs = read_cost_pairs(f.path().to_str().unwrap());
        assert_eq!(pairs, vec![(1.5, 2.5), (3.0, 4.0)]);
    }

    #[test]
    fn test_read_cost_pairs_ignores_invalid_lines() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "1.0 2.0").unwrap();
        writeln!(f, "invalid line here").unwrap();
        writeln!(f, "3.0 4.0").unwrap();
        let pairs = read_cost_pairs(f.path().to_str().unwrap());
        assert_eq!(pairs, vec![(1.0, 2.0), (3.0, 4.0)]);
    }
}

fn weighted_score(a: f64, b: f64, alpha: f64, beta: f64) -> f64 {
    alpha * a + beta * b
}

pub fn find_best_pair(pairs: &[(f64, f64)], alpha: f64, beta: f64) -> Option<((f64, f64), f64)> {
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