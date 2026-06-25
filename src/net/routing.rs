use ipnet::Ipv4Net;
use local_ip_address::local_ip;
use std::net::IpAddr;

/// Auto-detecta la subred local leyendo la IP activa del sistema
pub fn get_local_subnet() -> Option<Ipv4Net> {
    // local_ip() descubre tu IP principal de salida
    if let Ok(IpAddr::V4(ipv4)) = local_ip() {
        
        // Asumimos una máscara de red estándar /24 y truncamos a la IP base (.0)
        if let Ok(net) = Ipv4Net::new(ipv4, 24) {
            return Some(net.trunc());
        }
    }
    None
}