use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use std::sync::mpsc::Sender;
use crate::modulos::nombre_puertos::nombre_puerto;

use super::grab_banner::grab_banner;

pub fn escanear_rango(ip: IpAddr, inicio: u16, fin: u16, timeout: Duration, tx: Sender<String>) {

    for puerto in inicio..=fin {
        

        let direccion = SocketAddr::new(ip, puerto);
        

        if TcpStream::connect_timeout(&direccion, timeout).is_ok() {
            let banner = grab_banner(ip, puerto, timeout.as_millis() as u64);
            let texto_grab = match banner {
                Some(texto) => texto,
                None => "Banner no encontrado".to_string(),
            };

            match tx.send(format!("[OPEN] {} ({}) - {}", puerto, nombre_puerto(puerto),texto_grab)){
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error al enviar: {}", e);
                    return;
                }
            };
        }
    }
}