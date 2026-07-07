use std::net::IpAddr;
use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

pub fn grab_banner(ip: IpAddr, puerto: u16, timeout_ms: u64)-> Option<String>{

        let direccion = SocketAddr::new(ip, puerto);
        

        let mut servidor = TcpStream::connect_timeout
        (&direccion, Duration::from_millis(timeout_ms)).ok()?;

        servidor.set_read_timeout(Some(Duration::from_millis(timeout_ms))).ok()?;

        if puerto == 80 || puerto == 8080{
            servidor.write_all(b"HEAD / HTTP/1.0\r\n\r\n").ok()?;
        }
        let mut buffer = [0; 1024];
        let bytes_leidos = servidor.read(&mut buffer).ok()?;

        let respuesta = String::from_utf8_lossy(&buffer[..bytes_leidos]).trim().to_string();
        Some(respuesta)
}