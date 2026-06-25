use std::net::IpAddr;

// Usamos el parámetro 'valor' en lugar de args[3]
pub fn validar_fin(valor: &str, inicio: u16) -> u16 {
    match valor.parse::<u16>() {
        Ok(v) if v > inicio => v,
        Ok(_) => {
            eprintln!("El puerto final debe ser mayor al inicial");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("El puerto final es inválido");
            std::process::exit(1);
        }
    }
}

pub fn validar_hilos_vs_puertos(threads: u16, total: u16) {
    if threads > total {
        eprintln!("La cantidad de hilos no puede ser mayor que la cantidad de puertos");
        std::process::exit(1);
    }
}

pub fn validar_inicio(valor: &str) -> u16 {
    match valor.parse::<u16>() {
        Ok(v) if v > 0 => v,
        Ok(_) => {
            eprintln!("La cantidad de puertos debe ser mayor a 0");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("El puerto de inicio es inválido");
            std::process::exit(1);
        }
    }
}

pub fn validar_ip(ip: &str) -> IpAddr {
    match ip.parse::<IpAddr>() {
        Ok(ipvalida) => ipvalida,
        Err(_) => {
            eprintln!("Formato de IP inválida");
            std::process::exit(1);
        }
    }
}

pub fn validar_rango(inicio: u16, fin: u16) {
    if inicio >= fin {  
        eprintln!("El puerto inicial debe ser menor que el final");
        std::process::exit(1);
    }
}

pub fn validar_timeout_ms(valor: &str) -> u64 {
    match valor.parse::<u64>() {
        Ok(v) if v > 50 => v,
        Ok(_) => {
            eprintln!("El timeout debe ser mayor a 50ms");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("El timeout es inválido");
            std::process::exit(1);
        }
    }
}

pub fn validar_cantidad_hilos(valor: &str) -> u16 {
    match valor.parse::<u16>() {
        Ok(v) if v >= 1 && v < 20 => v,
        Ok(_) => {
            eprintln!("La cantidad de hilos debe ser mayor a 1 y menor a 20");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Valor inválido de hilos: {}", e);
            std::process::exit(1);
        }
    }
}