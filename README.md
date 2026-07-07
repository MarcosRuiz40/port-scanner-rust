# 🔍 Escáner de Puertos en Rust

Escáner de puertos concurrente desarrollado desde cero en Rust, utilizando threads y `TcpStream::connect_timeout` para realizar conexiones TCP con control de tiempo de espera.

## Explicación breve del proyecto: 

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

## 🏗️ Arquitectura
El proyecto está organizado de forma modular para separar responsabilidades y facilitar su mantenimiento.

src/
│
├── main.rs
│
└── modulos/
    ├── mod.rs
    ├── argumentos.rs       // Procesa los argumentos de la CLI
    ├── validaciones.rs     // Valida IP, puertos, etc.
    ├── cantidad_hilos.rs   // Calcula/distribuye el trabajo entre hilos
    ├── threads.rs          // Crea y gestiona los hilos
    ├── escaner.rs          // Lógica de conexión TCP
    └── nombre_puerto.rs    // Asocia puertos conocidos con servicios

## 🧠 ¿Cómo funciona?

1. El usuario proporciona:
- Dirección IP o hostname.
- Puerto inicial.
- Puerto final.
- Timeout de conexión.
- Cantidad de hilos.
2. El programa divide el rango de puertos entre múltiples hilos.
3. Cada hilo intenta establecer una conexión mediante:
TcpStream::connect_timeout(...)
4. Si la conexión es exitosa, el puerto se considera abierto y se envía el resultado al hilo principal mediante un canal MPSC.
5. Finalmente, se muestran todos los puertos abiertos junto con el servicio asociado cuando es conocido.

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

## 🎯 Objetivos del proyecto
Este proyecto fue desarrollado con fines educativos para comprender el funcionamiento de un escáner de puertos a bajo nivel, evitando depender de bibliotecas externas y profundizando en los fundamentos de Rust y la programación de redes.

## Mejoras futuras
- Implementación de una versión asíncrona utilizando Tokio.
- Exportación de resultados a JSON y CSV.
- Banner grabbing para identificar versiones de servicios.
- Resolución automática de nombres de host.
- Soporte para escaneo UDP.
- Configuración avanzada mediante argumentos CLI (clap).
- Benchmarks para comparar el rendimiento entre diferentes modelos de concurrencia.
- Suite de pruebas automatizadas.
