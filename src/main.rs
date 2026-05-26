use std::net::{SocketAddr, TcpStream};// Permite hacer conexiones TPC
use std::thread; // Permite crear hilos
use std::time::{Duration, Instant}; // permite manejar tiempos 
use std::sync::mpsc;// comunicación entre hilos
use std::sync::mpsc::Sender;
use std::env;
use std::net::IpAddr;
mod modulos;

use modulos::escaner::escanear_rango;
use modulos::threads::dividir_trabajo;

fn main() {
let tiempo_inicio = Instant::now(); // Inicio de tiempo
let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!("Uso: {} <IP> <inicio> <fin> <timeout_ms> <threads>", args[0]);
        eprintln!("Ejemplo: {} 192.168.0.1 1 1024 100 4", args[0]);
        return;
    }

let ip = args[1].clone();
let ip_2 = match ip.parse::<IpAddr>(){
    Ok(ipvalida)=> ipvalida,
    Err(e) => {
        eprintln!("Formato de IP inválida");
        return;
    }
};


let inicio: u16 = match args[2].parse() {
    Ok(v) if v > 0 => v,
    Ok( _ ) => {
        println!("La cantidad de puertos debe ser mayor a 0");
        return;
    }
    Err(e)=>{
        eprintln!("El puerto de inicio es inválido");
        return;
    }
};


let fin: u16 = match args[3].parse(){
    Ok(v) if v > inicio => v,
    Ok(_) => {
        println!("El puerto final debe ser mayor al inicial");
        return;
    }
    Err(e)=>{
        eprintln!("El puerto final es inválido");
        return;
    }
};

let timeout_ms: u64 = match args[4].parse(){
    Ok(v) if v > 50 => v,
    Ok(_) => {
        println!("El timeout debe ser mayor a 50ms ");
        return;
    }
    Err(e)=>{
        eprintln!("El timeout es inválido");
        return;
    }
};
let timeout = Duration :: from_millis(timeout_ms);

let threads:u16 = match args[5].parse(){
    Ok(v) if v >= 1 && v < 20 => v,
    Ok(_) => {
        println!("La cantidad de hilos debe ser mayor a 1 y menor a 20");
        return;
    }
    Err(e)=>{
        eprintln!("Valor inválido: {}", e);
        return;
    }
};

if inicio >= fin {
    eprintln!("El puerto inicial debe ser menor que el final");
    return;
}

let total = fin - inicio + 1;

if threads as usize > total as usize {
    eprintln!("La cantidad de hilos no puede ser mayor que la cantidad de puertos");
    return;
}

let tamaño = total/threads;


// Creamos un canal para comunicar los hilos con el hilo principal
let (tx,rx) = mpsc::channel();

    
    let mut hilos = Vec::new();

let hilos = dividir_trabajo(inicio,fin,threads,tamaño,ip_2,timeout,tx );


drop(tx); //Cerramos el canal principal

for mensaje in rx{
    println!("{}",mensaje);
}

for hilo in hilos{
   match hilo.join(){
    Ok(v)=> v,
    Err(e)=>{
        eprintln!("Fallo un hilo");
        return;
    }
   };
}

let fin_tiempo = tiempo_inicio.elapsed();// cortamos el tiempo 
println!("El escaneo se completo en {} segundos ({} ms)",fin_tiempo.as_secs(), fin_tiempo.as_millis()); // mostramos el tiempo transcurrido
}