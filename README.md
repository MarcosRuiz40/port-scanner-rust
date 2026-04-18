# 🔍 Escáner de Puertos en Rust

Escáner de puertos concurrente desarrollado en Rust utilizando `TcpStream`, hilos y manejo de timeout.

## 🚀 Características

- Escaneo de puertos concurrente mediante threads
- Uso de timeout para evitar bloqueos
- Implementación simple sin dependencias externas
- Rápido para pruebas en entornos locales

## 🧠 ¿Cómo funciona?

El programa divide el rango de puertos en múltiples hilos.  
Cada hilo intenta establecer una conexión TCP con un puerto específico utilizando `TcpStream::connect_timeout`.

- Si la conexión es exitosa → el puerto está abierto  
- Si falla → el puerto está cerrado o filtrado  

## ⚙️ Uso

```bash
cargo run
