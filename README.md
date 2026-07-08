# 🔍 Escáner de Puertos en Rust

Escáner de puertos concurrente desarrollado desde cero en Rust, utilizando threads y `TcpStream::connect_timeout` para realizar conexiones TCP con control de tiempo de espera.

## Explicación breve del proyecto: 

El programa recibe los parámetros desde la línea de comandos, valida la entrada, divide el rango de puertos entre múltiples hilos y utiliza TcpStream::connect_timeout para comprobar qué puertos están abiertos. Los resultados se comunican al hilo principal mediante un canal MPSC y se muestran junto con el servicio conocido y el banner cuando está disponible.

## 🚀 Features 
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
Desde la terminal el usuario debe ejecutar el comando cargo run -- 192.168.0.1 1 1024 100 4 (ip + puerto de inicio y final + timeout + hilos)

<img width="652" height="175" alt="image" src="https://github.com/user-attachments/assets/9908ee59-a0a0-46fb-96d3-5ee65cfd2b95" />

## Que aprendi 
- Ownership y Borrowing
- Organización modular del código
- Concurrencia con threads
- Comunicación entre hilos mediante MPSC
- Manejo de errores con Result y Option
- Networking con TcpStream
- Timeouts

## 🎯 Objetivos del proyecto
Este proyecto fue desarrollado con fines educativos para comprender el funcionamiento de un escáner de puertos a bajo nivel, evitando depender de bibliotecas externas y profundizando en los fundamentos de Rust y la programación de redes.

## Mejoras futuras
- Implementación de una versión asíncrona utilizando Tokio.
- Exportación de resultados a JSON y CSV.
- Resolución automática de nombres de host.
- Soporte para escaneo UDP.
- Configuración avanzada mediante argumentos CLI (clap).
- Suite de pruebas automatizadas.
