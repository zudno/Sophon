use colored::Colorize;
use tracing::{info, error};
use crate::net::tcp::scan_ports;

pub async fn execute(ip: &str, ports_str: &str) {
    info!("Ejecutando comando port-scan para la IP: {} con puertos: {}", ip, ports_str);
    
    // Parsear los puertos introducidos por el usuario
    let mut ports = vec![];
    for p in ports_str.split(',') {
        match p.trim().parse::<u16>() {
            Ok(port) => ports.push(port),
            Err(_) => {
                error!("Puerto inválido introducido: {}", p);
                println!("{} Error: El puerto '{}' no es un número válido. Ignorando.", "[-]".red(), p);
            }
        }
    }

    if ports.is_empty() {
        error!("No se proporcionaron puertos válidos para escanear.");
        println!("{} Error: No se proporcionaron puertos válidos para escanear.", "[-]".red());
        return;
    }

    println!("{} Iniciando escaneo de puertos en {}...", "[SOPHON]".blue().bold(), ip.cyan());
    
    // Llamar al motor TCP
    scan_ports(ip, &ports).await;
    
    println!("{} Escaneo de puertos completado.", "[SOPHON]".bright_green());
}
