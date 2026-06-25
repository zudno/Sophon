use colored::Colorize;
use ipnet::Ipv4Net;
use crate::net::routing::get_local_subnet;
use crate::net::icmp::sweep_network;

pub async fn ejecutar(red_str: Option<&str>) {
    println!("{} Configurando radar de Capa 3...", "[SOPHON]".blue().bold());
    
    let target_net: Ipv4Net = match red_str {
        // 1. Si el usuario ingresó una red manualmente, intentamos procesarla
        Some(red) => match red.parse() {
            Ok(net) => net,
            Err(_) => {
                println!("{} Error: Formato CIDR inválido.", "[-]".red());
                return;
            }
        },
        // 2. Si el usuario no puso nada, llamamos al auto-detector
        None => match get_local_subnet() {
            Some(net) => {
                println!("{} Red no especificada. Auto-detectada: {}", "[*]".yellow(), net.to_string().cyan());
                net
            },
            None => {
                println!("{} Error: No se pudo detectar una conexión de red activa.", "[-]".red());
                return;
            }
        }
    };

    // Lanzamos el motor ICMP asíncrono
    sweep_network(target_net).await;
}
