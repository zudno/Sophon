use ipnet::Ipv4Net;
use tokio::process::Command;
use tokio::task;
use colored::Colorize;
use tracing::info;

/// Realiza ping de forma concurrente a todos los hosts de una subred y retorna las IPs activas
pub async fn sweep_network(subnet: Ipv4Net) -> Vec<String> {
    info!("Iniciando barrido ICMP sobre la subred {}", subnet);
    println!("{} Iniciando barrido sobre {}...", "[SOPHON]".blue().bold(), subnet.to_string().bright_blue());
    
    let mut tasks = vec![];

    // FASE 1: Detección de Capa 3 y OS Fingerprinting
    for target_ip in subnet.hosts() {
        let ip_str = target_ip.to_string();
        
        let task = task::spawn(async move {
            if let Some(ttl) = ping_host(&ip_str).await {
                let os_guess = guess_os_from_ttl(ttl);
                println!("{} Host activo detectado: {:<15} | OS Probable: {} (TTL: {})", 
                    "[+]".bright_green(), 
                    ip_str.cyan(), 
                    os_guess.bright_magenta(),
                    ttl
                );
                Some(ip_str) // Devolvemos la IP para recolectarla
            } else {
                None
            }
        });
        
        tasks.push(task);
    }

    // Recolección de IPs activas
    let mut alive_hosts = vec![];
    for t in tasks {
        if let Ok(Some(ip)) = t.await {
            alive_hosts.push(ip);
        }
    }

    println!("------------------------------------------------");
    println!("{} Barrido completado. {} hosts activos encontrados.", "[SOPHON]".bright_green(), alive_hosts.len());
    info!("Barrido completado. Se encontraron {} hosts activos.", alive_hosts.len());

    alive_hosts
}

/// Llama al comando nativo del sistema operativo y extrae el TTL si responde
async fn ping_host(ip: &str) -> Option<u8> {
    let mut cmd = Command::new("ping");

    if cfg!(target_os = "windows") {
        cmd.args(["-n", "1", "-w", "500", ip]);
    } else if cfg!(target_os = "macos") {
        cmd.args(["-c", "1", "-W", "500", ip]);
    } else {
        cmd.args(["-c", "1", "-W", "1", ip]);
    }

    let output = cmd.output().await;

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout).to_lowercase();
        
        // Buscamos "ttl=" en la salida
        if let Some(ttl_idx) = stdout.find("ttl=") {
            // Extraemos los caracteres que siguen a "ttl="
            let start = ttl_idx + 4;
            let ttl_str: String = stdout[start..].chars().take_while(|c| c.is_ascii_digit()).collect();
            
            if let Ok(ttl) = ttl_str.parse::<u8>() {
                return Some(ttl);
            }
        }
    }
    
    None
}

/// Heurística básica de Fingerprinting basada en el TTL inicial
fn guess_os_from_ttl(ttl: u8) -> &'static str {
    // Los TTL suelen iniciar en 64 (Linux/Mac), 128 (Windows), o 255 (Cisco/Solaris).
    // Cada salto de red (router) disminuye el TTL en 1.
    if ttl <= 64 {
        "Linux / macOS / Unix"
    } else if ttl <= 128 {
        "Windows"
    } else {
        "Cisco / Equipos de Red"
    }
}