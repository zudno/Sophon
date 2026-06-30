use clap::Parser;
use colored::Colorize;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod commands;
mod net;

use cli::{Cli, Comandos};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let banner = r#"
  ██████  ▒█████   ██▓███   ██░ ██  ▒█████   ███▄    █ 
▒██    ▒ ▒██▒  ██▒▓██░  ██▒▓██░ ██▒▒██▒  ██▒ ██ ▀█   █ 
░ ▓██▄   ▒██░  ██▒▓██░ ██▓▒▒██▀▀██░▒██░  ██▒▓██  ▀█ ██▒
  ▒   ██▒▒██   ██░▒██▄█▓▒ ▒░▓█ ░██ ▒██   ██░▓██▒  ▐▌██▒
▒██████▒▒░ ████▓▒░▒██▒ ░  ░░▓█▒░██▓░ ████▓▒░▒██░   ▓██░
▒ ▒▓▒ ▒ ░░ ▒░▒░▒░ ▒▓▒░ ░  ░ ▒ ░░▒░▒░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
░ ░▒  ░ ░  ░ ▒ ▒░ ░▒ ░      ▒ ░▒░ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
░  ░  ░  ░ ░ ░ ▒  ░░        ░  ░░ ░░ ░ ░ ▒     ░   ░ ░ 
      ░      ░ ░            ░  ░  ░    ░ ░           ░ 
    "#;

    println!("{}", banner.bright_green().bold());

    let app = Cli::parse();

    match &app.comando {
        Comandos::PingSweep { red } => {
            commands::ping_sweep::ejecutar(red.as_deref()).await;
        }
        Comandos::PortScan { ip, ports } => {
            commands::port_scan::ejecutar(ip, ports).await;
        }
        Comandos::Arp => {
            commands::arp::ejecutar().await;
        }
    }
}