# 🔍 Escáner de Puertos en Rust

Escáner de puertos concurrente desarrollado desde cero en Rust, utilizando threads y `TcpStream::connect_timeout` para realizar conexiones TCP con control de tiempo de espera.

## Explicación breve del proyecto: 

El sistema, al recibir todos los datos que el usuario ingresa por la terminal, los organiza en segmentos que cumplen distintas funciones. Entre ellas se incluyen la división del trabajo mediante múltiples hilos, la asignación de tiempos de espera entre ejecuciones para evitar un uso excesivo de recursos, la posibilidad de definir un rango de puertos (especificando un puerto inicial y uno final) y, por último, la comunicación mediante el uso de MPSC.

## 🚀 Características

- Escaneo concurrente mediante múltiples hilos
- Uso de timeout para evitar bloqueos en conexiones
- Implementación sin dependencias externas (std)
- Enfoque en aprendizaje de networking a bajo nivel
- Implementación de Grab banners
- Validaciones

## Features 
- Multithreading
- Configurable timeout
- Port range scanning
- Service detection (common ports)
- Arquitectura modular
- grab_banner
- Medición del tiempo de ejecución
- Salida clara por consola

## 💡 Qué resuelve

Este proyecto busca entender cómo funcionan los escáneres de red a bajo nivel, implementando manualmente la lógica de conexión y concurrencia en Rust.

## 🏗️ Arquitectura
El proyecto está organizado de forma modular para separar responsabilidades y facilitar su mantenimiento.
<img width="158" height="245" alt="image" src="https://github.com/user-attachments/assets/a203e6e2-f183-4485-a6e1-729eceda7b07" />


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

<img width="652" height="175" alt="image" src="https://github.com/user-attachments/assets/9908ee59-a0a0-46fb-96d3-5ee65cfd2b95" />

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
- Resolución automática de nombres de host.
- Soporte para escaneo UDP.
- Configuración avanzada mediante argumentos CLI (clap).
- Benchmarks para comparar el rendimiento entre diferentes modelos de concurrencia.
- Suite de pruebas automatizadas.
