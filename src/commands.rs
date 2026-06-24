use std::process::Command;

pub fn handle(input: &str) -> String {
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd").args(&["/C", input]).output();

    #[cfg(not(target_os = "windows"))] // Linux, macOS, etc.
    let output = Command::new("sh").arg("-c").arg(input).output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{}{}", stdout, stderr)
        }
        Err(e) => e.to_string(),
    }
}
