
pub fn validar_argumentos(args: &Vec<String>){
    if args.len() != 6 {
        eprintln!("Uso: {} <IP> <inicio> <fin> <timeout_ms> <threads>", args[0]);
        eprintln!("Ejemplo: {} 192.168.0.1 1 1024 100 4", args[0]);
        return;
    }
}