use std::time::{Duration, Instant}; 
use std::sync::mpsc;
use std::env;
use std::fs;
use chrono::Local;

mod modulos;
use crate::modulos::modelos::EscaneoPuerto;
use modulos::threads::dividir_trabajo;
use modulos::argumentos::validar_argumentos;
// Agrupamos los imports del módulo validaciones
use modulos::validaciones::{
    validar_ip, validar_inicio, validar_fin, 
    validar_cantidad_hilos, validar_hilos_vs_puertos, 
    validar_rango, validar_timeout_ms
};

use crate::modulos::modelos::PuertoAbierto;

fn main() {
let tiempo_inicio = Instant::now(); // Inicio de tiempo
let args: Vec<String> = env::args().collect();
validar_argumentos(&args);

let _ip = args[1].clone();
let ip_valida = validar_ip(&args[1]);


let inicio = validar_inicio(&args[2]);

let fin = validar_fin(&args[3], inicio);

let timeout_ms = validar_timeout_ms(&args[4]);
let timeout = Duration :: from_millis(timeout_ms);


let threads = validar_cantidad_hilos(&args[5]);

validar_rango(inicio, fin);

let total = fin - inicio + 1;

validar_hilos_vs_puertos(threads, total);

let tamaño = total/threads;

println!("==================================================");
println!("[*] Iniciando escaneo en el objetivo: {}", ip_valida);
println!("[*] Rango de puertos: {} - {} (Total: {})", inicio, fin, total);
println!("[*] Configuración: {} hilos | Timeout: {}ms", threads, timeout_ms);
println!("==================================================");
println!("[+] Escaneando...");



// Creamos un canal para comunicar los hilos con el hilo principal
let (tx,rx) = mpsc::channel();
    
let hilos = dividir_trabajo(inicio, fin, threads, tamaño, ip_valida, timeout, tx);

let mut puerto_abiertos: Vec<PuertoAbierto> = Vec::new() ;

for mensaje in rx{
    println!("[OPEN] {} ({}) - {}",mensaje.puerto,mensaje.servicio,mensaje.banner);
    puerto_abiertos.push(mensaje);
}

let escaneo = EscaneoPuerto{
    ip: ip_valida,
    fecha: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),

    puertos_abiertos : puerto_abiertos
};

for hilo in hilos{
   match hilo.join(){
    Ok(v)=> v,
    Err(_e)=>{
        eprintln!("Fallo un hilo");
        return;
    }
   };
}

let json = serde_json::to_string_pretty(&escaneo).unwrap();
match fs::write("Escaneo.json", json){
    Ok(_)=> println!("[+] El archivo: Escaneo.json se ha guardado correctamente"),
    Err(e)=> eprintln!("Se ha producido un error al guardar el archivo: {}", e),
}

let fin_tiempo = tiempo_inicio.elapsed();// cortamos el tiempo 
println!("==================================================");
println!("[+] Escaneo se ha realizado con exito");
println!("El escaneo se completo en {} segundos ({} ms)",fin_tiempo.as_secs(), fin_tiempo.as_millis()); // mostramos el tiempo transcurrido
println!("==================================================");
}