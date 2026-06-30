use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "sophon")]
#[command(version, about = "Herramienta de reconocimiento de red", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Realiza un barrido ICMP para descubrir dispositivos vivos en una red
    PingSweep {
        /// Red objetivo en formato CIDR (ej. 192.168.1.0/24). Si se omite, se escaneará la red local automáticamente.
        #[arg(short, long)]
        network: Option<String>,
    },
    /// Escanea puertos TCP de un dispositivo específico
    PortScan {
        /// IP objetivo para el escaneo de puertos
        #[arg(short, long)]
        ip: String,
        /// Lista de puertos separados por coma (ej. 22,80,443)
        #[arg(short, long, default_value = "21,22,23,25,53,80,110,135,443,3389")]
        ports: String,
    },
    /// Muestra la tabla de caché ARP y los fabricantes de los dispositivos
    Arp,
}