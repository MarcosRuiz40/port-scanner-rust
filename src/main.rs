use core::time;
use std::fs::read;
use std::net::{SocketAddr, TcpStream};// Permite hacer conexiones TPC
use std::thread; // Permite crear hilos
use std::time::Duration; // permite manejar tiempos 
use std::env::args; // Usamos el entorno de RUST
use std::sync::mpsc;// comunicación entre hilos
use std::sync::mpsc::Sender;
use std::env;
use std::net::IpAddr;

fn nombre_puerto(puerto: u16) -> &'static str {
    match puerto {
        20 => "FTP-DATA",
        21 => "FTP",
        22 => "SSH",
        23 => "TELNET",
        25 => "SMTP",
        53 => "DNS",
        67 => "DHCP-SERVER",
        68 => "DHCP-CLIENT",
        69 => "TFTP",
        80 => "HTTP",
        110 => "POP3",
        119 => "NNTP",
        123 => "NTP",
        135 => "RPC",
        137 => "NETBIOS-NS",
        138 => "NETBIOS-DGM",
        139 => "NETBIOS-SSN",
        143 => "IMAP",
        161 => "SNMP",
        179 => "BGP",
        194 => "IRC",
        389 => "LDAP",
        443 => "HTTPS",
        445 => "SMB",
        465 => "SMTPS",
        500 => "ISAKMP",
        514 => "SYSLOG",
        515 => "LPD",
        520 => "RIP",
        587 => "SMTP-SUBMISSION",
        636 => "LDAPS",
        989 => "FTPS-DATA",
        990 => "FTPS",
        993 => "IMAPS",
        995 => "POP3S",
        1433 => "MSSQL",
        _ => "UNKNOWN"
    }
}

fn escanear_rango(ip: IpAddr, inicio: u16, fin: u16, timeout: Duration, tx: Sender<String>) {
    for puerto in inicio..=fin {
        let direccion = SocketAddr::new(ip, puerto);

        if TcpStream::connect_timeout(&direccion, timeout).is_ok() {
            match tx.send(format!("[OPEN] {} ({})", puerto, nombre_puerto(puerto))){
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error al enviar: {}", e);
                    return;
                }
            };
        }
    }
}

fn main() {
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
    Ok(_)=>{
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

let total = (fin - inicio + 1);

if threads as usize > total as usize {
    eprintln!("La cantidad de hilos no puede ser mayor que la cantidad de puertos");
    return;
}

let tamaño = total/threads;


// Creamos un canal para comunicar los hilos con el hilo principal
let (tx,rx) = mpsc::channel();

    
    let mut hilos = Vec::new();

for i in 0..threads{
            
    let inicio_thread = inicio + i * tamaño;
    let fin_thread = if i == threads - 1 {
        fin
    }else{
        inicio_thread + tamaño - 1
    };
    let ip_clone = ip_2;
    let tx_clone = tx.clone();

    let hilo = thread::spawn(move||{
        escanear_rango(ip_clone, inicio_thread, fin_thread, timeout, tx_clone);
    });
    hilos.push(hilo);
}


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

}