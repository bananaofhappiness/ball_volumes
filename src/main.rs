use statrs::function::gamma::gamma;
use rand::Rng;
// use std::io;
fn main() {
    for dimension in 2..11 {
        let volume = calculate_volume(dimension);
        let real_volume = real_volume(dimension);
        let accuracy = calculate_accuracy(volume, real_volume);
        println!("Количество измерений: {dimension} | Объем: {volume} | Реальный объем: {real_volume} | Точность: {accuracy}%");
    }    
}

fn generate_point(dimensions: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut coordinates = Vec::with_capacity(dimensions);
    for _ in 0..dimensions {
        coordinates.push(rng.gen_range(-1.0..=1.0));
    }
    return coordinates;
}

fn point_is_in_sphere(point: &Vec<f64>) -> bool {
    let mut sum: f64 = 0.0;
    for coordinate in point {
        sum += coordinate.powf(2.0);
    }
    return sum.powf(1.0/point.len() as f64)<1.0;
}

fn calculate_volume(dimensions: usize) -> f64 {
    let mut overall_points = 0;
    let mut points_in_sphere = 0;
    for _ in 0..10000000 {
        let point = generate_point(dimensions);
        overall_points += 1;
        if point_is_in_sphere(&point) {
            points_in_sphere += 1;
        }
    }
    let ratio_of_points_in_sphere = points_in_sphere as f64 / overall_points as f64;
    let volume = ratio_of_points_in_sphere * 2_i32.pow(dimensions as u32) as f64;
    return volume;
}

fn real_volume(dimensions: usize) -> f64 {
    let real_volume = (std::f64::consts::PI.powf(dimensions as f64/2.0))/gamma((dimensions as f64/2.0)+1.0);
    return real_volume;
}

fn calculate_accuracy(volume: f64, real_volume: f64) -> f64 {
    let error_rate = ((volume-real_volume).abs()/real_volume) * 100f64;
    let accuracy = 100.00 - error_rate;
    return accuracy;
}