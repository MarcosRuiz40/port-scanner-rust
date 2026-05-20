pub fn escanear_rango(ip: IpAddr, inicio: u16, fin: u16, timeout: Duration, tx: Sender<String>) {

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