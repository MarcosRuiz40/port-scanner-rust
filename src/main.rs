use std::net::{SocketAddr, TcpStream};// Permite hacer conexiones TPC
use std::thread; // Permite crear hilos
use std::time::Duration; // permite manejar tiempos 
//use std::env; // Usamos el entorno de RUST
use std::sync::mpsc;// comunicación entre hilos
fn main() {
let ip = "127.0.0.1";
let inicio = 1;
let fin = 500;
let inicio2 = 501;
let fin2 = 1000;
let timeout = Duration::from_millis(30);

// Creamos un canal para comunicar los hilos con el hilo principal
let (tx,rx) = mpsc::channel();

// Clonamos el transmisor para que cada hilo tenga su propia copia
let tx1 = tx.clone();
let tx2 = tx.clone();

let hilo1 = thread::spawn(move ||{
    for puerto in inicio..=fin{
         let ip_puerto = format!("{}:{}",ip ,puerto);
         let direccion: SocketAddr = ip_puerto.parse().unwrap();
         if TcpStream::connect_timeout(&direccion, timeout).is_ok(){
            tx1.send(format!("[OPEN]{} ",puerto)).unwrap();

            
            
    }
}
});

let hilo2 = thread::spawn(move ||{
    for puerto in inicio2..=fin2{
         let ip_puerto = format!("{}:{}",ip ,puerto);
         let direccion: SocketAddr = ip_puerto.parse().unwrap();
         if TcpStream::connect_timeout(&direccion, timeout).is_ok(){
             tx2.send(format!("[OPEN]{} ",puerto)).unwrap();
    }
}
});

drop(tx); //Cerramos el canal principal

for mensaje in rx{
    println!("{}",mensaje);
}



hilo1.join().unwrap();
hilo2.join().unwrap();



}
