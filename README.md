# 🔍 Escáner de Puertos en Rust

Escáner de puertos concurrente desarrollado desde cero en Rust, utilizando threads y `TcpStream::connect_timeout` para realizar conexiones TCP con control de tiempo de espera.

## 🚀 Características

- Escaneo concurrente mediante múltiples hilos
- Uso de timeout para evitar bloqueos en conexiones
- Implementación sin dependencias externas (std)
- Enfoque en aprendizaje de networking a bajo nivel

## 💡 Qué resuelve

Este proyecto busca entender cómo funcionan los escáneres de red a bajo nivel, implementando manualmente la lógica de conexión y concurrencia en Rust.

## 🧠 ¿Cómo funciona?

El programa divide el rango de puertos en distintos hilos.  
Cada hilo intenta establecer una conexión TCP con un puerto específico:

- ✔ Si la conexión es exitosa → el puerto está abierto  
- ❌ Si falla → el puerto está cerrado o filtrado  

## ⚙️ Uso

```bash
cargo run
