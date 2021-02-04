use std::io;

fn main() {
    let mut weight = String::new();
    io::stdin().read_line(&mut weight);
    println!("Weight on Mars: {}", calculate_weight_on_mars(100.0));
}

fn calculate_weight_on_mars(earth_weight: f32) -> f32 {
    (earth_weight / 9.81) * 3.711
}