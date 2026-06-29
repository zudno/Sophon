use ipnet::Ipv4Net;
use tokio::process::Command;
use tokio::task;
use colored::Colorize;

/// Lanza 254 pings simultáneos usando el motor asíncrono de Tokio
pub async fn sweep_network(subnet: Ipv4Net) {
    println!("{} Iniciando barrido masivo sobre {}...", "[SOPHON]".blue().bold(), subnet.to_string().bright_red());
    
    let mut tasks = vec![];

    // FASE 1: Detección de Capa 3 y 4
    for target_ip in subnet.hosts() {
        let ip_str = target_ip.to_string();
        
        let task = task::spawn(async move {
            let is_alive = ping_host(&ip_str).await;
            if is_alive {
                println!("{} Host vivo detectado: {}", "[+]".bright_green(), ip_str.cyan());
                crate::net::tcp::scan_ports(&ip_str).await;
                Some(ip_str) // Devolvemos la IP para la Fase 2
            } else {
                None
            }
        });
        
        tasks.push(task);
    }

    // Recolectamos a los sobrevivientes
    let mut alive_hosts = vec![];
    for t in tasks {
        if let Ok(Some(ip)) = t.await {
            alive_hosts.push(ip);
        }
    }

    // FASE 2: Extracción de Identidades de Hardware (Capa 2)
    println!("------------------------------------------------");
    println!("{} Infiltrando caché ARP para extraer firmas de hardware...", "[SOPHON]".blue().bold());
    
    let arp_table = crate::net::mac::get_arp_table().await;
    
    for ip in &alive_hosts {
        if let Some(mac) = arp_table.get(ip) {
            let manufacturer = crate::net::mac::get_manufacturer(mac);
            
            // Formato condicional para destacar cuando sí reconocemos al fabricante
            let mfg_colored = if manufacturer.contains("Desconocido") {
                manufacturer.bright_black() // Gris oscuro si no lo conocemos
            } else {
                manufacturer.bright_yellow().bold() // Amarillo brillante si es un hit
            };

            println!("    {} IP: {:<15} | MAC: {} | Fabricante: {}", 
                "[@]".bright_cyan(), 
                ip.cyan(), 
                mac.bright_magenta(), 
                mfg_colored
            );
        }
    }

    println!("------------------------------------------------");
    println!("{} Barrido completado. {} dispositivos analizados en todas las capas.", "[SOPHON]".bright_green(), alive_hosts.len());
}

/// Llama al comando nativo del sistema operativo de forma multiplataforma
async fn ping_host(ip: &str) -> bool {
    let mut cmd = Command::new("ping");

    if cfg!(target_os = "windows") {
        cmd.args(["-n", "1", "-w", "500", ip]);
    } else if cfg!(target_os = "macos") {
        cmd.args(["-c", "1", "-W", "500", ip]);
    } else {
        cmd.args(["-c", "1", "-W", "1", ip]);
    }

    let output = cmd.output().await;

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_lowercase();
            stdout.contains("ttl=")
        }
        Err(_) => false,
    }
}