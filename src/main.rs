use clap::Parser;
use colored::Colorize;

mod cli;
mod commands;
mod net;

use cli::{Cli, Comandos};

#[tokio::main]
async fn main() {
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
        Comandos::Discover { red } => {
            // Delegamos la ejecución al comando Discover pasándole la red (si el usuario la escribió)
            commands::discover::ejecutar(red.as_deref()).await;
        }
    }
}