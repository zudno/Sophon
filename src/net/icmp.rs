use ipnet::Ipv4Net;
use tokio::process::Command;
use tokio::task;
use colored::Colorize;

/// Lanza 254 pings simultáneos usando el motor asíncrono de Tokio
pub async fn sweep_network(subnet: Ipv4Net) {
    println!("{} Iniciando barrido masivo sobre {}...", "[SOPHON]".blue().bold(), subnet.to_string().bright_red());
    
    let mut tasks = vec![];

    // Recorremos todas las IPs de la subred
    for target_ip in subnet.hosts() {
        let ip_str = target_ip.to_string();
        
        // Abrimos un hilo asíncrono por cada IP
        let task = task::spawn(async move {
            let is_alive = ping_host(&ip_str).await;
            if is_alive {
                println!("{} Host vivo detectado: {}", "[+]".bright_green(), ip_str.cyan());
            }
            is_alive
        });
        
        tasks.push(task);
    }

    // Esperamos a que todos los pings terminen de regresar
    let mut found = 0;
    for t in tasks {
        if let Ok(true) = t.await {
            found += 1;
        }
    }

    println!("------------------------------------------------");
    println!("{} Barrido completado. {} dispositivos respondieron al eco.", "[SOPHON]".bright_green(), found);
}

/// Llama al comando nativo del sistema operativo de forma multiplataforma
async fn ping_host(ip: &str) -> bool {
    let mut cmd = Command::new("ping");

    // Detectamos el SO en tiempo de ejecución para usar los parámetros correctos
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
            // TTL indica que el paquete rebotó exitosamente
            stdout.contains("ttl=")
        }
        Err(_) => false,
    }
}