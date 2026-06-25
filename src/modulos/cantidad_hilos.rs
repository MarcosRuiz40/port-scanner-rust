pub fn validar_cantidad_hilos(valor: &str)->u16{
    match valor.parse::<u16>(){
    Ok(v) if v >= 1 && v < 20 => v,
    Ok(_) => {
        println!("La cantidad de hilos debe ser mayor a 1 y menor a 20");
        std::process::exit(1);
    }
    Err(e)=>{
        eprintln!("Valor inválido: {}", e);
        std::process::exit(1);
    }
}
}