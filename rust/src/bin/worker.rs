use rand::Rng;
use serde::Serialize;
use reqwest::Client;
use std::time::Duration;

#[derive(Serialize)]
struct ResultMsg {
    darts_inside_circle: u64,
    darts_total: u64,
    worker_id: String,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    // Detectar el nombre del worker (si es un contenedor de Docker, dirá su ID)
    let worker_name = std::env::var("HOSTNAME").unwrap_or_else(|_| "Host-Local-Edgar".into());
    
    let darts_to_throw = 1_000_000;
    let mut inside_circle = 0;
    let mut rng = rand::thread_rng();

    println!("⚙️  [Worker: {}] Iniciando cálculo de {} dardos...", worker_name, darts_to_throw);

    // Lógica matemática de Monte Carlo
    for _ in 0..darts_to_throw {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    let payload = ResultMsg {
        darts_inside_circle: inside_circle,
        darts_total: darts_to_throw,
        worker_id: worker_name,
    };

    // Intentar enviar al Coordinador (usa localhost por ahora, luego será la IP de la VPN)
    println!("📡 Enviando resultados al Coordinador...");
    
    match client.post("http://127.0.0.1:3000/submit")
        .json(&payload)
        .timeout(Duration::from_secs(5))
        .send()
        .await {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("✅ Éxito: El Coordinador recibió los datos.");
                } else {
                    println!("⚠️  Servidor respondió con error: {:?}", resp.status());
                }
            },
            Err(e) => println!("❌ No se pudo contactar al Coordinador: {}. ¿Está encendido?", e),
        }
}
