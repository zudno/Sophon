use colored::Colorize;
use ipnet::Ipv4Net;
use tracing::{info, error};
use crate::net::routing::get_local_subnet;
use crate::net::icmp::sweep_network;

pub async fn execute(network_str: Option<&str>) {
    info!("Iniciando barrido de red ICMP...");
    
    let target_net: Ipv4Net = match network_str {
        Some(network) => match network.parse() {
            Ok(net) => net,
            Err(_) => {
                error!("Formato CIDR inválido provisto por el usuario: {}", network);
                println!("{} Error: Formato CIDR inválido.", "[-]".red());
                return;
            }
        },
        None => match get_local_subnet() {
            Some(net) => {
                info!("Red auto-detectada: {}", net);
                println!("{} Red auto-detectada: {}", "[*]".yellow(), net.to_string().cyan());
                net
            },
            None => {
                error!("No se pudo detectar una conexión de red activa.");
                println!("{} Error: No se pudo detectar una conexión de red activa.", "[-]".red());
                return;
            }
        }
    };

    sweep_network(target_net).await;
}
