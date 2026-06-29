use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use colored::Colorize;

/// Escanea los puertos y extrae inteligencia de la Capa 7 (Banner Grabbing)
pub async fn scan_ports(ip: &str) {
    let common_ports = [21, 22, 23, 25, 53, 80, 110, 135, 443, 3389];
    let mut open_ports_info = vec![];

    for port in common_ports {
        let addr = format!("{}:{}", ip, port);
        
        // Fase 1: Intentamos el apretón de manos TCP (Capa 4)
        let conn_attempt = TcpStream::connect(&addr);
        
        if let Ok(Ok(mut stream)) = timeout(Duration::from_millis(500), conn_attempt).await {
            // Fase 2: Si el puerto está abierto, interrogamos (Capa 7)
            let mut port_info = port.to_string();
            
            if let Some(banner) = grab_banner(&mut stream, port, ip).await {
                port_info = format!("{} ({})", port, banner.bright_black());
            }
            
            open_ports_info.push(port_info);
        }
    }

    if !open_ports_info.is_empty() {
        println!("    {} Servicios activos en {}: [{}]", 
            "[►]".bright_purple(), 
            ip.cyan(), 
            open_ports_info.join(", ")
        );
    }
}

/// Función táctica para extraer el banner dependiendo del protocolo
async fn grab_banner(stream: &mut TcpStream, port: u16, ip: &str) -> Option<String> {
    // Si es HTTP (80), tenemos que provocar al servidor enviando una petición primero
    if port == 80 {
        let req = format!("HEAD / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", ip);
        let _ = stream.write_all(req.as_bytes()).await;
    }

    let mut buffer = [0; 256];
    
    // Le damos al servicio máximo 500ms para responder a nuestro interrogatorio
    let read_attempt = stream.read(&mut buffer);
    
    if let Ok(Ok(bytes_read)) = timeout(Duration::from_millis(500), read_attempt).await {
        if bytes_read > 0 {
            // Convertimos los bytes crudos a texto
            let raw_banner = String::from_utf8_lossy(&buffer[..bytes_read]);
            
            // Limpiamos el texto: tomamos solo la primera línea y quitamos espacios extra
            let clean_banner = raw_banner.lines().next().unwrap_or("").trim().to_string();
            
            // Para HTTP, a veces la primera línea es solo "HTTP/1.1 200 OK", 
            // pero nos sirve para confirmar el servicio web.
            if !clean_banner.is_empty() {
                return Some(clean_banner);
            }
        }
    }
    
    None
}