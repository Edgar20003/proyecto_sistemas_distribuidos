use serde::{Deserialize, Serialize};

// Definición de mensajes (Estructuras requeridas por la rúbrica)
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    task_id: u32,
    darts_to_throw: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultMsg {
    task_id: u32,
    darts_inside_circle: u64,
    worker_id: String,
}

fn main() {
    println!("🚀 [Coordinador] Iniciando sistema distribuido de Monte Carlo...");
    
    // 1. Coordinador genera tarea Dummy
    let dummy_task = Task { task_id: 1, darts_to_throw: 100_000 };
    println!("📤 [Coordinador] Enviando tarea dummy al worker: {:?}", dummy_task);

    // 2. Simulación: El worker recibe y procesa
    println!("   ⚙️ [Worker-Abraham] Recibí tarea {}. Simulando cálculo de Pi...", dummy_task.task_id);
    let dummy_result = ResultMsg { 
        task_id: dummy_task.task_id, 
        darts_inside_circle: 78_540, 
        worker_id: "Worker-Abraham".to_string() 
    };

    // 3. Coordinador recibe el resultado
    println!("📥 [Coordinador] Resultado recibido de {}: {:?}", dummy_result.worker_id, dummy_result);
    println!("✅ [Sistema] Prueba 'Hello Distributed' completada exitosamente.");
}
