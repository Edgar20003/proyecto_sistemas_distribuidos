# Usamos la imagen oficial de Rust en su versión ligera
FROM rust:1.76-slim

# Creamos el directorio de trabajo dentro del contenedor
WORKDIR /app

# Como es un worker genérico (por ahora), lo mantenemos encendido esperando tareas
CMD ["tail", "-f", "/dev/null"]

