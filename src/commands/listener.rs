use colored::Colorize;
use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tracing::{info, error};

/// Escucha en un puerto TCP y redirige stdin/stdout bidireccionalmente
/// con la primera conexión entrante (equivalente a `nc -lvnp <port>`).
pub async fn execute(port: u16) {
    let bind_addr = format!("0.0.0.0:{}", port);

    println!(
        "{} Iniciando listener en {}...",
        "[SOPHON]".blue().bold(),
        bind_addr.cyan()
    );
    info!("Iniciando listener TCP en {}", bind_addr);

    // Intentar abrir el socket de escucha
    let listener = match TcpListener::bind(&bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("No se pudo abrir el puerto {}: {}", port, e);
            println!(
                "{} Error: No se pudo abrir el puerto {}: {}",
                "[-]".red(),
                port,
                e
            );
            return;
        }
    };

    println!(
        "{} Escuchando en el puerto {}. Esperando conexión...",
        "[*]".yellow().bold(),
        port.to_string().bright_green()
    );

    // Aceptar la primera conexión entrante
    let (socket, peer_addr) = match listener.accept().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Error al aceptar conexión: {}", e);
            println!("{} Error al aceptar conexión: {}", "[-]".red(), e);
            return;
        }
    };

    println!(
        "{} Conexión recibida desde {}",
        "[+]".bright_green().bold(),
        peer_addr.to_string().bright_purple()
    );
    info!("Conexión entrante aceptada desde {}", peer_addr);

    // Dividir el socket TCP en lectura y escritura independientes
    let (mut sock_read, mut sock_write) = socket.into_split();

    // Tarea 1: Leer del socket remoto -> escribir en stdout (lo que envía la víctima)
    let remote_to_stdout = tokio::spawn(async move {
        let mut stdout = io::stdout();
        let mut buf = vec![0u8; 4096];
        loop {
            match sock_read.read(&mut buf).await {
                Ok(0) => {
                    // Conexión cerrada por el otro extremo
                    break;
                }
                Ok(n) => {
                    if stdout.write_all(&buf[..n]).await.is_err() {
                        break;
                    }
                    let _ = stdout.flush().await;
                }
                Err(_) => break,
            }
        }
    });

    // Tarea 2: Leer de stdin (nuestro teclado) -> escribir en el socket remoto
    let stdin_to_remote = tokio::spawn(async move {
        let mut stdin = io::stdin();
        let mut buf = vec![0u8; 1024];
        loop {
            match stdin.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    if sock_write.write_all(&buf[..n]).await.is_err() {
                        break;
                    }
                    let _ = sock_write.flush().await;
                }
                Err(_) => break,
            }
        }
    });

    // Esperar a que cualquiera de las dos tareas termine (la conexión se cierra)
    tokio::select! {
        _ = remote_to_stdout => {
            println!(
                "\n{} Conexión cerrada por el host remoto.",
                "[!]".yellow().bold()
            );
        }
        _ = stdin_to_remote => {
            println!(
                "\n{} Sesión finalizada por el usuario.",
                "[!]".yellow().bold()
            );
        }
    }

    info!("Sesión del listener finalizada para {}", peer_addr);
    println!(
        "{} Listener finalizado.",
        "[SOPHON]".bright_green()
    );
}
