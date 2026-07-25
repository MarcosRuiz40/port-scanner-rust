use std::net::IpAddr;
use serde::Serialize;

#[derive(Serialize)]
pub struct PuertoAbierto{
    pub puerto: u16,
    pub servicio:String,
    pub banner:String
}

#[derive(Serialize)]
pub struct EscaneoPuerto{
    pub ip: IpAddr,
    pub fecha: String,
    pub puertos_abiertos:Vec<PuertoAbierto>
}