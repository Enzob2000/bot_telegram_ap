use std::io::Write;
use std::process::Command;
use std::{fs::File, path::Path};

pub fn lector() -> &'static str {
    // Ejecutar el comando ocrs image.png
    let output = Command::new("ocrs")
        .arg("2776250164-2.png")
        .output()
        .expect("Failed to execute process");

    // Verificar si el comando fue exitoso
    if !output.status.success() {
        // Convertir la salida en texto
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    println!("texto {}", stdout);

    if stdout.contains("Agotado") && stdout.contains("Reducido") {
        return "Agotado";
    }

    if stdout.contains("Los numeros solicitados estan agotados") {
        return "numero";
    }

    return "";
}
