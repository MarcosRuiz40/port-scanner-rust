fn dividir_trabajo(inicio: u16, fin: u16, threads: u16, tamaño: u16, ip: IpAddr, timeout: Duration, tx: Sender<String>)-> Vec<thread::JoinHandle<()>>{
    let mut hilos = Vec::new();
    for i in 0..threads{
            
    let inicio_thread = inicio + i * tamaño;
    let fin_thread = if i == threads - 1 {
        fin
    }else{
        inicio_thread + tamaño - 1
    };
    let ip_clone = ip;
    let tx_clone = tx.clone();

    let hilo = thread::spawn(move||{
        escanear_rango(ip_clone, inicio_thread, fin_thread, timeout, tx_clone);
    });
    hilos.push(hilo);
}
drop(tx); //Cerramos el canal principal
hilos
}