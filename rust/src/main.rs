use std::env;

// Definición de estructuras (Requisito del Doc B)
#[derive(Debug)]
struct Task {
    id: u32,
    instruccion: String,
}

#[derive(Debug)]
struct Result {
    task_id: u32,
    estado: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let rol = if args.len() > 1 { &args[1] } else { "coordinador" };

    println!("🚀 Iniciando nodo con rol: {}", rol);

    if rol == "coordinador" {
        let dummy_task = Task { id: 1, instruccion: String::from("Calcular fragmento Mandelbrot") };
        println!("[Coordinador] Enviando tarea a worker: {:?}", dummy_task);
        
        // Simular espera de red
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        println!("[Coordinador] Resultado recibido del worker: Tarea {} completada con éxito.", dummy_task.id);
    } else {
        println!("[Worker] Esperando tareas del coordinador...");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        let dummy_result = Result { task_id: 1, estado: String::from("Completado") };
        println!("[Worker] Procesando tarea... enviando resultado: {:?}", dummy_result);
    }
}
