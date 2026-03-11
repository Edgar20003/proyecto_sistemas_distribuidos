use rand::Rng;
use serde::{Deserialize, Serialize};

fn main() {
    let darts_to_throw = 1_000_000;
    println!("⚙️ [Worker] Lanzando {} dardos (Monte Carlo)...", darts_to_throw);

    let mut inside_circle = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..darts_to_throw {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }
    println!("✅ Completado. Puntos dentro del círculo: {}", inside_circle);
}
