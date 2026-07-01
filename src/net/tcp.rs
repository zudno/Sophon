use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use colored::Colorize;
use tracing::info;

/// Escanea una lista de puertos TCP e intenta extraer información del banner
pub async fn scan_ports(ip: &str, ports: &[u16]) {
    info!("Iniciando escaneo de puertos en {}", ip);
    let mut open_ports_info = vec![];

    for &port in ports {
        let addr = format!("{}:{}", ip, port);
        
        let conn_attempt = TcpStream::connect(&addr);
        
        // Timeout extendido para permitir que la conexión se establezca correctamente
        if let Ok(Ok(mut stream)) = timeout(Duration::from_millis(1000), conn_attempt).await {
            // Intentamos extraer un banner (Service Version / HTTP Title)
            let port_info = if let Some(banner) = grab_banner(&mut stream, port, ip).await {
                format!("{}/tcp abierto -> {}", port, banner)
            } else {
                format!("{}/tcp abierto", port)
            };
            
            open_ports_info.push(port_info);
        }
    }

    if !open_ports_info.is_empty() {
        println!("    {} Puertos encontrados en {}:", "[►]".bright_purple(), ip.cyan());
        for info in &open_ports_info {
            println!("      - {}", info);
        }
        info!("Puertos abiertos encontrados en {}: {:?}", ip, open_ports_info);
    } else {
        println!("    {} No se encontraron puertos abiertos en {}", "[!]".yellow(), ip.cyan());
        info!("No se encontraron puertos abiertos en {}", ip);
    }
}

/// Dirige el escaneo dependiendo de si es un protocolo web o estándar
async fn grab_banner(stream: &mut TcpStream, port: u16, ip: &str) -> Option<String> {
    if port == 80 || port == 8080 {
        return interrogate_http(stream, ip).await;
    }

    // Para protocolos estándar (22 SSH, 21 FTP, etc.), leemos la presentación inicial
    let mut buffer = vec![0; 1024];
    let read_attempt = stream.read(&mut buffer);
    
    // Algunos servicios tardan en enviar su banner
    if let Ok(Ok(bytes_read)) = timeout(Duration::from_millis(1500), read_attempt).await {
        if bytes_read > 0 {
            let raw_banner = String::from_utf8_lossy(&buffer[..bytes_read]);
            let clean_banner = raw_banner.lines().next().unwrap_or("").trim().to_string();
            
            if !clean_banner.is_empty() {
                return Some(clean_banner);
            }
        }
    }
    
    None
}

/// Ejecuta un mini-script nativo que envía un GET HTTP, extrae el Servidor y el Título de la página
async fn interrogate_http(stream: &mut TcpStream, ip: &str) -> Option<String> {
    // Simulamos ser un cliente válido para no ser bloqueados inmediatamente
    let req = format!("GET / HTTP/1.1\r\nHost: {}\r\nUser-Agent: SOPHON-Scanner/1.0\r\nConnection: close\r\n\r\n", ip);
    let _ = stream.write_all(req.as_bytes()).await;

    let mut buffer = vec![0; 4096]; // Buffer grande para cargar el HTML
    let read_attempt = stream.read(&mut buffer);

    if let Ok(Ok(bytes_read)) = timeout(Duration::from_millis(2000), read_attempt).await {
        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            
            let mut server_version = "HTTP Desconocido";
            let mut title = "Sin Título";

            // 1. Extraemos la cabecera 'Server:'
            for line in response.lines() {
                let lower_line = line.to_lowercase();
                if lower_line.starts_with("server:") {
                    server_version = line[7..].trim(); // Tomamos lo que sigue a "Server:"
                }
            }

            // 2. Extraemos el texto dentro de <title>
            let resp_lower = response.to_lowercase();
            if let Some(start_idx) = resp_lower.find("<title>") {
                if let Some(end_idx) = resp_lower[start_idx..].find("</title>") {
                    // +7 por el tamaño de la palabra "<title>"
                    let extracted = &response[start_idx + 7 .. start_idx + end_idx];
                    title = extracted.trim();
                }
            }

            return Some(format!("Servidor: {} | Título web: '{}'", server_version.bright_yellow(), title.bright_cyan()));
        }
    }

    None
}