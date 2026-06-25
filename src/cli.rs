use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "sophon")]
#[command(version, about = "Herramienta de reconocimiento de red", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub comando: Comandos,
}

#[derive(Subcommand, Debug)]
pub enum Comandos {
    /// Descubre dispositivos vivos en una red
    Discover {
        /// Red objetivo en formato CIDR (ej. 192.168.1.0/24). Si se omite, SOPHON escaneará tu red local automáticamente.
        #[arg(short, long)]
        red: Option<String>,
    },
}