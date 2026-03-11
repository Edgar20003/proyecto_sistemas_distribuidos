use axum::{routing::post, Json, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

// Estructura para acumular los resultados de todos los integrantes
struct AppState {
    total_inside: Mutex<u64>,
    total_thrown: Mutex<u64>,
}

#[derive(Deserialize)]
struct ResultMsg {
    darts_inside_circle: u64,
    darts_total: u64,
    worker_id: String,
}

#[tokio::main]
async fn main() {
    // Estado compartido seguro entre hilos
    let shared_state = Arc::new(AppState {
        total_inside: Mutex::new(0),
        total_thrown: Mutex::new(0),
    });

    let app = Router::new()
        .route("/submit", post({
            let state = Arc::clone(&shared_state);
            move |Json(payload): Json<ResultMsg>| async move {
                submit_handler(payload, state).await
            }
        }));

    // El coordinador escuchará en el puerto 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    println!("🚀 [COORDINADOR] Servidor activo en http://localhost:3000");
    println!("📊 Esperando datos de los workers...");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn submit_handler(
    payload: ResultMsg,
    state: Arc<AppState>,
) -> &'static str {
    let mut inside = state.total_inside.lock().unwrap();
    let mut total = state.total_thrown.lock().unwrap();

    *inside += payload.darts_inside_circle;
    *total += payload.darts_total;

    let pi_approx = 4.0 * (*inside as f64) / (*total as f64);
    let error = ((pi_approx - std::f64::consts::PI).abs() / std::f64::consts::PI) * 100.0;

    println!("\n--- 📊 REPORTE DE PROGRESO ---");
    println!("👤 Nodo: {}", payload.worker_id);
    println!("🎯 Dardos en este envío: {}", payload.darts_total);
    println!("📈 PI Estimado: {:.10}", pi_approx);
    println!("📉 Margen de Error: {:.6}%", error);
    println!("-----------------------------\n");

    "✅ Recibido"
}
