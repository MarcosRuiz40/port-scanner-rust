# 🔍 Escáner de Puertos en Rust

Escáner de puertos concurrente desarrollado desde cero en Rust, utilizando threads y `TcpStream::connect_timeout` para realizar conexiones TCP con control de tiempo de espera.

Explicación breve del proyecto: 
El sistema, al recibir todos los datos que el usuario ingresa por la terminal, los organiza en segmentos que cumplen distintas funciones. Entre ellas se incluyen la división del trabajo mediante múltiples hilos, la asignación de tiempos de espera entre ejecuciones para evitar un uso excesivo de recursos, la posibilidad de definir un rango de puertos (especificando un puerto inicial y uno final) y, por último, la comunicación mediante el uso de MPSC.

## 🚀 Características

- Escaneo concurrente mediante múltiples hilos
- Uso de timeout para evitar bloqueos en conexiones
- Implementación sin dependencias externas (std)
- Enfoque en aprendizaje de networking a bajo nivel

## Features 
- Multithreading
- Configurable timeout
- Port range scanning
- Service detection (common ports)

## 💡 Qué resuelve

Este proyecto busca entender cómo funcionan los escáneres de red a bajo nivel, implementando manualmente la lógica de conexión y concurrencia en Rust.

## 🧠 ¿Cómo funciona?

El programa divide el rango de puertos en distintos hilos.  
Cada hilo intenta establecer una conexión TCP con un puerto específico:

- ✔ Si la conexión es exitosa → el puerto está abierto  
- ❌ Si falla → el puerto está cerrado o filtrado  

## ⚙️ Uso

```bash
cargo run -- 192.168.0.1 1 1024 100 4
SALIDA:
[OPEN] 80 (HTTP)
[OPEN] 443 (HTTPS)
```
## Que aprendi 
- Concurrencia en RUST
- Manejo de errores con result y match
- Networking básico
- Timeouts

## Mejoras futuras
- Mejor manejo de errores con Result
- Modularización del código
- Exportar resultados a archivo
- Soporte para rangos grandes optimizado
