use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {

    let _timeout = Duration::from_millis(200);

    let hilo1 = thread::spawn(||{
        let ip = "127.0.0.1";
        for puerto in 1..500{
            let ip_puerto = format!("{}:{}", ip, puerto);
            let direccion = &ip_puerto.parse().unwrap();
                match TcpStream::connect_timeout(&direccion, _timeout) {
                Ok(_) => println!("Puerto {} abierto",puerto),
                Err(_) => (),      
            }       
        }

    });

    let hilo2 = thread::spawn(||{
        let ip = "127.0.0.1";
        for puerto in 500..1000{
            let ip_puerto = format!("{}:{}", ip, puerto);
            let direccion = &ip_puerto.parse().unwrap();
                match TcpStream::connect_timeout(&direccion, _timeout) {
                Ok(_) => println!("Puerto {} abierto",puerto),
                Err(_) => (),
            }    
        }
    });    

    let _ = hilo1.join().unwrap();
    let _ = hilo2.join().unwrap();
}
