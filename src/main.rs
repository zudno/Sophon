use clap::Parser;
use colored::Colorize;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod commands;
mod net;

use cli::{Cli, Commands};

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

    match &app.command {
        Commands::PingSweep { network } => {
            commands::ping_sweep::execute(network.as_deref()).await;
        }
        Commands::PortScan { ip, ports } => {
            commands::port_scan::execute(ip, ports).await;
        }
        Commands::Arp => {
            commands::arp::execute().await;
        }
    }
}