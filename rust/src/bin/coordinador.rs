use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ResultMsg {
    task_id: u32,
    darts_inside_circle: u64,
    worker_id: String,
}

fn main() {
    println!("🚀 [Nodo Coordinador] Iniciando agregador de resultados...");
    let total_darts: u64 = 1_000_000;
    
    // Simulación de un resultado recibido
    let mock_result = ResultMsg {
        task_id: 1,
        darts_inside_circle: 785_400,
        worker_id: "Worker-Edgar".to_string(),
    };

    let pi_approx = 4.0 * (mock_result.darts_inside_circle as f64) / (total_darts as f64);
    println!("📥 Resultado de {}: {} puntos dentro.", mock_result.worker_id, mock_result.darts_inside_circle);
    println!("📊 Valor aproximado de PI: {}", pi_approx);
}
