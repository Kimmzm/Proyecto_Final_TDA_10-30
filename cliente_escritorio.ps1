[console]::InputEncoding = [System.Text.Encoding]::UTF8; [console]::OutputEncoding = [System.Text.Encoding]::UTF8
Add-Type -AssemblyName Microsoft.VisualBasic
Add-Type -AssemblyName System.Windows.Forms

# Titulo de la aplicacion de escritorio
$AppTitle = "Sistema de Registro de Empleados - Cliente de Escritorio"

[System.Windows.Forms.MessageBox]::Show("Bienvenido al Cliente de Escritorio. Presione Aceptar para iniciar el registro del nuevo empleado.", $AppTitle, [System.Windows.Forms.MessageBoxButtons]::OK, [System.Windows.Forms.MessageBoxIcon]::Information)

# 1. Ventanas emergentes de captura de datos nitidas (Inputs de Escritorio)
$EmpNo = [Microsoft.VisualBasic.Interaction]::InputBox("Ingrese el Numero de Empleado (Ej: 500001):", $AppTitle, "")
if ([string]::IsNullOrEmpty($EmpNo)) { exit }

$FirstName = [Microsoft.VisualBasic.Interaction]::InputBox("Ingrese el Primer Nombre del Empleado:", $AppTitle, "")
if ([string]::IsNullOrEmpty($FirstName)) { exit }

$LastName = [Microsoft.VisualBasic.Interaction]::InputBox("Ingrese el Apellido del Empleado:", $AppTitle, "")
if ([string]::IsNullOrEmpty($LastName)) { exit }

$Gender = [Microsoft.VisualBasic.Interaction]::InputBox("Ingrese el Genero (M o F):", $AppTitle, "")
if ([string]::IsNullOrEmpty($Gender)) { exit }

$BirthDate = [Microsoft.VisualBasic.Interaction]::InputBox("Fecha de Nacimiento (Formato: AAAA-MM-DD):", $AppTitle, "1995-06-15")
if ([string]::IsNullOrEmpty($BirthDate)) { exit }

$HireDate = [Microsoft.VisualBasic.Interaction]::InputBox("Fecha de Contratacion (Formato: AAAA-MM-DD):", $AppTitle, "2026-05-24")
if ([string]::IsNullOrEmpty($HireDate)) { exit }

# 2. Empaquetar los datos capturados en formato JSON estructurado
$EmployeePayload = @{
    emp_no     = [int]$EmpNo
    birth_date = $BirthDate
    first_name = $FirstName
    last_name  = $LastName
    gender     = $Gender
    hire_date  = $HireDate
} | ConvertTo-Json

# 3. Consumir la API REST de Rust apuntando al Servidor de Cesar
$Url = "http://192.168.201.107:8080/api/employees"

try {
    # Realiza la peticion mandando el JSON directo al backend en Rust
    $Response = Invoke-RestMethod -Uri $Url -Method Post -Body $EmployeePayload -ContentType "application/json; charset=utf-8"
    
    # Mostrar confirmacion visual en el escritorio de Windows
    [System.Windows.Forms.MessageBox]::Show("Respuesta del Servidor Rust: `n$Response", "Exito de Registro", [System.Windows.Forms.MessageBoxButtons]::OK, [System.Windows.Forms.MessageBoxIcon]::Information)
}
catch {
    [System.Windows.Forms.MessageBox]::Show("No se pudo conectar con el servidor de Rust de Cesar. Asegurese de estar en la misma red Wi-Fi y que el contenedor Docker este activo.`nError: $_", "Error Critico", [System.Windows.Forms.MessageBoxButtons]::OK, [System.Windows.Forms.MessageBoxIcon]::Error)
}