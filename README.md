# 🔍 Escáner de puertos en Rust

Un sencillo escáner de puertos concurrente escrito en Rust que utiliza `TcpStream`, hilos y gestión de tiempos de espera.

## 🚀 Características

- Escaneo de puertos concurrente mediante hilos
- Tiempo de espera configurable por conexión
- Escaneo rápido de hosts locales y remotos
- Ligero y con mínimas dependencias (solo std)

## 🧠 Cómo funciona

El programa divide el rango de puertos en varios hilos.

Cada hilo intenta conectarse a un puerto usando `TcpStream::connect_timeout`.

Si la conexión es exitosa, el puerto está abierto.

Si falla, el puerto está cerrado o filtrado.

## ⚙️ Uso

```bash
cargo run
```

## 💻 Tecnologias utilizadas
![Rust](https://img.shields.io/badge/language-Rust-orange)

## 💪 Aprendizajes 
- Uso de la libreria TcpStream, Thread, Duration
