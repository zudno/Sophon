use colored::Colorize;
use tracing::info;
use crate::net::mac::{get_arp_table, get_manufacturer};

pub async fn ejecutar() {
    info!("Ejecutando comando arp local");
    println!("{} Leyendo caché ARP del sistema...", "[SOPHON]".blue().bold());
    
    let arp_table = get_arp_table().await;
    
    if arp_table.is_empty() {
        println!("{} La tabla ARP local está vacía.", "[*]".yellow());
        info!("La tabla ARP local está vacía.");
        return;
    }

    println!("------------------------------------------------");
    for (ip, mac) in &arp_table {
        let manufacturer = get_manufacturer(mac);
        
        let mfg_colored = if manufacturer.contains("Desconocido") {
            manufacturer.bright_black()
        } else {
            manufacturer.bright_yellow().bold()
        };

        println!("    {} IP: {:<15} | MAC: {} | Fabricante: {}", 
            "[@]".bright_cyan(), 
            ip.cyan(), 
            mac.bright_magenta(), 
            mfg_colored
        );
    }
    println!("------------------------------------------------");
    println!("{} {} entradas ARP encontradas.", "[SOPHON]".bright_green(), arp_table.len());
    info!("Comando arp finalizado, {} entradas encontradas.", arp_table.len());
}
