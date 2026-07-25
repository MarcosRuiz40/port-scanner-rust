use std::net::{SocketAddr, TcpStream};
use std::net::IpAddr;
use std::time::Duration;
use std::sync::mpsc::Sender;
use crate::modulos::modelos::PuertoAbierto;
use crate::modulos::nombre_puertos::nombre_puerto;

use super::grab_banner::grab_banner;

pub fn escanear_rango(ip: IpAddr, inicio: u16, fin: u16, timeout: Duration, tx: Sender<PuertoAbierto>) {

    

    for puerto in inicio..=fin {
        

        let direccion = SocketAddr::new(ip, puerto);
        

        if TcpStream::connect_timeout(&direccion, timeout).is_ok() {
            let banner = grab_banner(ip, puerto, timeout.as_millis() as u64);
            let texto_grab = match banner {
                Some(texto) => texto,
                None => "Banner no encontrado".to_string(),
            };

let _puerto_abierto = PuertoAbierto{
                                    puerto: puerto,
                                    servicio:nombre_puerto(puerto).to_string(),
                                    banner:texto_grab.clone()
                                    };

            match tx.send(PuertoAbierto{puerto:puerto, servicio:nombre_puerto(puerto).to_string(), banner:texto_grab.clone()}){
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error al enviar: {}", e);
                    return;
                }
            };
        }
    }
}