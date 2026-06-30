use std::collections::HashMap;
use tokio::process::Command;
use tracing::error;

/// Obtiene la tabla ARP del sistema operativo y retorna un mapa de IP a MAC.
pub async fn get_arp_table() -> HashMap<String, String> {
    let mut arp_map = HashMap::new();
    
    // Ejecutamos el comando nativo del sistema para leer el caché
    match Command::new("arp").arg("-a").output().await {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                
                let mut ip = String::new();
                let mut mac = String::new();
                
                // Heurística ligera multiplataforma para extracción
                for part in parts {
                    if part.matches('.').count() == 3 {
                        ip = part.replace("(", "").replace(")", ""); // Limpiamos formato Linux
                    } else if part.matches('-').count() == 5 || part.matches(':').count() == 5 {
                        mac = part.replace("-", ":").to_uppercase(); // Normalizamos a formato estándar
                    }
                }
                
                // Filtramos MACs de broadcast o vacías
                if !ip.is_empty() && !mac.is_empty() && !mac.contains("FF:FF:FF:FF:FF:FF") {
                    arp_map.insert(ip, mac);
                }
            }
        },
        Err(e) => {
            error!("Error al ejecutar el comando ARP: {}", e);
        }
    }
    
    arp_map
}

/// Base de datos OUI (Organizationally Unique Identifier) codificada en duro
pub fn get_manufacturer(mac: &str) -> &str {
    let prefix = if mac.len() >= 8 { &mac[0..8] } else { "" };
    
    // Detectar Direcciones Administradas Localmente (LAA) / MACs Aleatorias
    if mac.len() >= 2 {
        let second_char = mac.chars().nth(1).unwrap();
        if second_char == '2' || second_char == '6' || second_char == 'A' || second_char == 'E' {
            return "MAC Aleatoria (Privacidad / Dispositivo Móvil)";
        }
    }

    match prefix {
        // Infraestructura de Red y Seguridad
        "80:80:2C" => "Fortinet (Firewall / Router)",
        "F4:92:BF" => "Ubiquiti Networks (Access Point / Switch)",
        "C0:74:AD" => "Grandstream Networks (Teléfono VoIP)",
        "F0:4C:D5" => "Maxlinear, Inc. (Chip de red)",
        
        // Servidores y Virtualización
        "BC:24:11" => "Proxmox Virtual Environment",
        "00:50:56" | "00:0C:29" | "00:05:69" => "VMware (Máquina Virtual)",
        "08:00:27" => "Oracle VirtualBox",
        
        // PCs, Laptops y Componentes
        "34:17:EB" | "48:4D:7E" => "Dell Inc.",
        "5C:BA:EF" => "Foxconn / HP (Chongqing Fugui)",
        "94:53:30" => "Foxconn (Hon Hai Precision)",
        "08:9D:F4" => "Intel Corporate",
        "04:7C:16" => "MSI (Micro-Star INTL)",
        
        // IoT y Smart Home
        "B8:27:EB" | "DC:A6:32" | "E4:5F:01" => "Raspberry Pi",
        "24:6F:28" | "EC:FA:BC" | "30:AE:A4" => "Espressif (Microcontrolador IoT)",
        
        // Dispositivos Legacy sin MAC Aleatoria
        "E0:D4:64" | "54:E3:42" | "00:10:E3" => "Samsung",
        "CC:50:E3" | "F0:18:98" => "Apple, Inc.", 
        
        _ => "Fabricante Desconocido",
    }
}