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

    // FASE 1: Detección de Capa 3
    for target_ip in subnet.hosts() {
        let ip_str = target_ip.to_string();
        
        let task = task::spawn(async move {
            let is_alive = ping_host(&ip_str).await;
            if is_alive {
                println!("{} Host activo detectado: {}", "[+]".bright_green(), ip_str.cyan());
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