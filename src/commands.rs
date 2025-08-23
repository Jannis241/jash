use std::process::Command;

pub fn handle(input: &String) -> String {
    let output = Command::new("sh").arg("-c").arg(input).output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Beide zusammengeben
            format!("{}{}", stdout, stderr)
        }
        Err(e) => e.to_string(), // Fehler beim Starten des Befehls
    }
}
