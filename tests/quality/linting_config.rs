use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Linting failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}