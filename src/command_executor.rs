use std::process::Command;
use std::path::Path;

pub fn run_git_clone(url: &str, destination: &str) -> Result<(), String> {
    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(destination)
        .output()
        .map_err(|e| format!("Failed to execute git command: {}", e))?;

    if output.status.success() {
        println!("Repository cloned successfully.");
        Ok(())
    } else {
        Err(format!(
            "Error cloning repository: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub fn set_executable_permission(file_path: &Path) -> Result<(), String> {
    let output = Command::new("chmod")
        .arg("+x")
        .arg(file_path)
        .output()
        .map_err(|_| "Failed to set permissions for the script.")?;

    if output.status.success() {
        println!("Permissions set successfully.");
        Ok(())
    } else {
        Err("Error setting permissions".to_string())
    }
}

pub fn run_bash_script(script_path: &Path, working_dir: &Path) -> Result<(), String> {
    let status = Command::new("bash")
        .arg(script_path)
        .current_dir(working_dir)
        .status()
        .map_err(|e| format!("Failed to execute script: {}", e))?;

    if status.success() {
        println!("Script executed successfully.");
        Ok(())
    } else {
        Err(format!("Script failed with exit code: {:?}", status.code()))
    }
}
