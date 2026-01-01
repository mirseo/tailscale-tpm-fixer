use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn remove_tailscale_files() -> Result<(), String> {
    // Stop Tailscale processes first
    stop_tailscale_processes()?;

    let username = std::env::var("USERNAME").unwrap_or_else(|_| "".to_string());
    let user_appdata = format!(r"C:\Users\{}\AppData\Local\Tailscale", username);

    let paths = vec![
        r"C:\ProgramData\Tailscale".to_string(),
        user_appdata,
    ];

    remove_files_owned(&paths)
}

#[cfg(target_os = "linux")]
pub fn remove_tailscale_files() -> Result<(), String> {
    // Stop Tailscale processes first
    stop_tailscale_processes()?;

    let paths = vec![
        "/var/lib/tailscale/tailscaled.state".to_string(),
    ];

    remove_files_owned(&paths)
}

#[cfg(target_os = "windows")]
fn stop_tailscale_processes() -> Result<(), String> {
    println!("  Stopping Tailscale processes...");

    let processes = vec!["tailscale.exe", "tailscaled.exe"];

    for process in processes {
        let output = Command::new("taskkill")
            .args(&["/F", "/IM", process])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("  [✓] Stopped: {}", process);
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    // Check if it's a "not found" error (process wasn't running)
                    if stderr.contains("not found") || stderr.contains("not running") {
                        println!("  [-] Not running: {}", process);
                    } else if stderr.contains("Access is denied") || stderr.contains("access denied") {
                        return Err("[!] Insufficient permissions. Please run with administrator privileges.".to_string());
                    } else {
                        println!("  [!] Failed to stop {}: {}", process, stderr.trim());
                    }
                }
            }
            Err(e) => {
                if is_permission_error(&e) {
                    return Err("[!] Insufficient permissions. Please run with administrator privileges.".to_string());
                }
                println!("  [!] Error stopping {}: {}", process, e);
            }
        }
    }

    println!();
    Ok(())
}

#[cfg(target_os = "linux")]
fn stop_tailscale_processes() -> Result<(), String> {
    println!("  Stopping Tailscale processes...");

    // Try to stop the service first
    let systemctl_result = Command::new("systemctl")
        .args(&["stop", "tailscaled"])
        .output();

    match systemctl_result {
        Ok(result) => {
            if result.status.success() {
                println!("  [✓] Stopped: tailscaled service");
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                if stderr.contains("Permission denied") || stderr.contains("access denied") {
                    return Err("[!] Insufficient permissions. Please run with administrator privileges.".to_string());
                }
                println!("  [-] Could not stop tailscaled service");
            }
        }
        Err(e) => {
            if is_permission_error(&e) {
                return Err("[!] Insufficient permissions. Please run with administrator privileges.".to_string());
            }
            println!("  [-] systemctl not available");
        }
    }

    // Also kill any running processes
    let processes = vec!["tailscale", "tailscaled"];

    for process in processes {
        let output = Command::new("pkill")
            .arg(process)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("  [✓] Stopped: {}", process);
                } else {
                    println!("  [-] Not running: {}", process);
                }
            }
            Err(e) => {
                if is_permission_error(&e) {
                    return Err("[!] Insufficient permissions. Please run with administrator privileges.".to_string());
                }
                println!("  [!] Error stopping {}: {}", process, e);
            }
        }
    }

    println!();
    Ok(())
}

fn remove_files_owned(paths: &[String]) -> Result<(), String> {
    let mut had_errors = false;
    let mut permission_error = false;

    for path in paths {
        if Path::new(path).exists() {
            let result = if Path::new(path).is_dir() {
                fs::remove_dir_all(path)
            } else {
                fs::remove_file(path)
            };

            match result {
                Ok(_) => {
                    println!("  [✓] Removed: {}", path);
                }
                Err(e) => {
                    if is_permission_error(&e) {
                        permission_error = true;
                        println!("  [!] Permission denied: {}", path);
                    } else {
                        had_errors = true;
                        println!("  [!] Failed to remove {}: {}", path, e);
                    }
                }
            }
        } else {
            println!("  [-] Not found: {}", path);
        }
    }

    if permission_error {
        return Err("[!] Insufficient permissions. Please run with administrator privileges.".to_string());
    }

    if had_errors {
        return Err("Some files could not be removed.".to_string());
    }

    Ok(())
}

fn is_permission_error(error: &std::io::Error) -> bool {
    use std::io::ErrorKind;
    matches!(error.kind(), ErrorKind::PermissionDenied)
}

pub fn show_completion_message() {
    println!("\n  Completed. Please re-register your device to reactivate the machine.\n");
}
