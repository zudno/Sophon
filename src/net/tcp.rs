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
        
        // Fase 1: Intentamos establecer conexión TCP
        let conn_attempt = TcpStream::connect(&addr);
        
        if let Ok(Ok(mut stream)) = timeout(Duration::from_millis(500), conn_attempt).await {
            // Fase 2: Si el puerto está abierto, intentamos leer el banner
            let mut port_info = port.to_string();
            
            if let Some(banner) = grab_banner(&mut stream, port, ip).await {
                port_info = format!("{} ({})", port, banner.bright_black());
            }
            
            open_ports_info.push(port_info);
        }
    }

    if !open_ports_info.is_empty() {
        println!("    {} Puertos abiertos en {}: [{}]", 
            "[►]".bright_purple(), 
            ip.cyan(), 
            open_ports_info.join(", ")
        );
        info!("Puertos abiertos encontrados en {}: {:?}", ip, open_ports_info);
    } else {
        println!("    {} No se encontraron puertos abiertos en {}", "[!]".yellow(), ip.cyan());
        info!("No se encontraron puertos abiertos en {}", ip);
    }
}

/// Extrae el banner de un servicio TCP, enviando una petición HTTP si es el puerto 80
async fn grab_banner(stream: &mut TcpStream, port: u16, ip: &str) -> Option<String> {
    // Si es HTTP (80 o 443), enviamos una petición básica para obtener respuesta
    if port == 80 {
        let req = format!("HEAD / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", ip);
        let _ = stream.write_all(req.as_bytes()).await;
    }

    let mut buffer = [0; 256];
    
    // Esperamos la respuesta del servicio con un timeout de 500ms
    let read_attempt = stream.read(&mut buffer);
    
    if let Ok(Ok(bytes_read)) = timeout(Duration::from_millis(500), read_attempt).await {
        if bytes_read > 0 {
            // Convertimos los bytes a texto
            let raw_banner = String::from_utf8_lossy(&buffer[..bytes_read]);
            
            // Limpiamos el texto: tomamos la primera línea y quitamos espacios en blanco
            let clean_banner = raw_banner.lines().next().unwrap_or("").trim().to_string();
            
            if !clean_banner.is_empty() {
                return Some(clean_banner);
            }
        }
    }
    
    None
}