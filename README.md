# Proyecto de Sistemas Distribuidos

## Descripción General
Este proyecto implementa un sistema distribuido en Rust para calcular el conjunto de Mandelbrot, corriendo sobre una infraestructura de contenedores Docker interconectados mediante una VPN privada (WireGuard) con topología Hub-and-Spoke.

## Requisitos de Software
* Linux (Ubuntu/Debian) o WSL2.
* WireGuard (`wireguard`, `iproute2`, `openresolv`).
* Docker y Docker Compose.
* Rust (`rustup`, `cargo`).

## Instrucciones de Despliegue

### 1. Levantar la VPN (WireGuard)
1. Colocar las plantillas de configuración sanitizadas en `/etc/wireguard/wg0.conf`.
2. Reemplazar las variables `<Llave_Privada>` y `<Llave_Publica>` por las correspondientes de cada nodo.
3. Ejecutar: `sudo wg-quick up wg0`

### 2. Desplegar Contenedores (Workers)
1. Navegar a `configuraciones_sanitizadas/docker/`.
2. Ejecutar: `docker compose up -d --build`
3. Verificar con: `docker ps`

### 3. Compilar y Ejecutar el Sistema en Rust
1. Navegar al directorio `rust/`.
2. Para iniciar el coordinador: `cargo run -- coordinador`
3. Para iniciar un worker: `cargo run -- worker`

## Notas y Supuestos
* No se suben llaves privadas a este repositorio por seguridad.
* Se asume que el Hub (10.10.10.1) tiene el puerto UDP 51820 expuesto al exterior.
