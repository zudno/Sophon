# SOPHON

SOPHON es una herramienta de reconocimiento y diagnóstico de red rápida y modular escrita en Rust. Está diseñada para administradores de red y profesionales de ciberseguridad, permitiendo realizar tareas de descubrimiento de hosts, escaneo de puertos y análisis de caché ARP de forma eficiente y concurrente.

## Características

* **Ping Sweep (`ping-sweep`)**: Descubrimiento de hosts vivos en una red local (o subred remota) utilizando envíos ICMP asíncronos concurrentes. Soporta autodescubrimiento de la red en la que te encuentras.
* **Escaneo de Puertos (`port-scan`)**: Análisis de puertos TCP a una IP específica. Soporta escaneo concurrente, banner grabbing (extracción de cabeceras HTTP/TCP) y permite listas de puertos personalizables.
* **Tabla ARP (`arp`)**: Lectura de la caché ARP local para correlacionar direcciones IP con direcciones MAC. Incluye una base de datos integrada para identificar a los fabricantes de las tarjetas de red (OUI).

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (cargo)
- Privilegios de red locales (algunos escaneos y la resolución ARP dependen de las tablas nativas del sistema operativo).

## Instalación y Compilación

Clona el repositorio y compílalo usando Cargo:

```bash
git clone https://github.com/tu-usuario/sophon.git
cd sophon
cargo build --release
```

El ejecutable optimizado estará disponible en `target/release/sophon`.

## Uso

Puedes ver la ayuda general de la herramienta ejecutando:

```bash
cargo run -- --help
```

### 1. Barrido de Red (Ping Sweep)
Descubre dispositivos activos. Si omites el parámetro `--red`, SOPHON autodetectará tu subred local.

```bash
# Autodetectar y escanear red local
cargo run -- ping-sweep

# Escanear una subred específica en formato CIDR
cargo run -- ping-sweep --red 192.168.1.0/24
```

### 2. Escaneo de Puertos (Port Scan)
Revisa qué puertos TCP están abiertos en una máquina destino.

```bash
# Escanear los puertos más comunes por defecto
cargo run -- port-scan --ip 192.168.1.15

# Escanear puertos específicos
cargo run -- port-scan --ip 192.168.1.15 --ports 22,80,443,8080
```

### 3. Caché ARP
Muestra las entradas de la tabla ARP de tu sistema y el fabricante del hardware asociado.

```bash
cargo run -- arp
```

## Logging y Depuración

SOPHON utiliza `tracing` para el manejo de logs internos. Por defecto verás mensajes informativos limpios (`INFO`). Si deseas depurar el comportamiento de la red, puedes ajustar el nivel de log mediante variables de entorno:

```bash
# En Linux/macOS
RUST_LOG=debug cargo run -- ping-sweep

# En Windows (PowerShell)
$env:RUST_LOG="debug"; cargo run -- ping-sweep
```

## Ética y Uso Legal

Esta herramienta está diseñada únicamente para propósitos de diagnóstico, auditoría y seguridad defensiva en redes de tu propiedad o en las que poseas autorización explícita para auditar. El uso no autorizado de herramientas de escaneo en redes de terceros puede ser considerado un delito.
