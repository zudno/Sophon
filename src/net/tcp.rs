use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use colored::Colorize;

/// Escanea una lista de puertos comunes en una IP dada usando TCP
pub async fn scan_ports(ip: &str) {
    // Lista de los "Top 10" puertos más comunes a investigar
    // 21 (FTP), 22 (SSH), 23 (Telnet), 25 (SMTP), 53 (DNS), 80 (HTTP), 110 (POP3), 135 (RPC), 443 (HTTPS), 3389 (RDP)
    let common_ports = [21, 22, 23, 25, 53, 80, 110, 135, 443, 3389];
    let mut open_ports = vec![];

    for port in common_ports {
        // Formato requerido para conectar: "192.168.1.73:80"
        let addr = format!("{}:{}", ip, port);
        
        // Intentamos establecer la conexión de Capa 4 con un timeout de 500ms
        let conn_attempt = TcpStream::connect(&addr);
        
        if let Ok(Ok(_)) = timeout(Duration::from_millis(500), conn_attempt).await {
            // Si el bloque Ok() doble se cumple, el puerto respondió al saludo
            open_ports.push(port);
        }
    }

    // Si encontramos al menos un puerto abierto, lo reportamos en la terminal
    if !open_ports.is_empty() {
        println!("    {} Puertos abiertos en {}: {:?}", "[►]".bright_purple(), ip.cyan(), open_ports);
    }
}