#!/bin/bash
echo "🧹 Limpiando entorno..."
sudo docker compose down

echo "🦀 Compilando binarios de Rust..."
cd rust
cargo build --release

echo "🐳 Levantando infraestructura de Workers (4 contenedores)..."
cd ../docker
sudo docker compose up -d --build

echo "✅ Sistema listo. Los workers locales están trabajando."
sudo docker ps
