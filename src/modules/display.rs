use std::io::{self, Write};

pub fn show_logo() {
    println!(r#"
 _____ _____ _____
|_   _|_   _|  ___|
  | |   | | | |_
  | |   | | |  _|
  |_|   |_| |_|

  tailscale-tpm-fixer

  This tool deletes Tailscale files associated with the TPM from your system.

  After the process is complete, please re-register your device to reactivate the machine.

  (See https://tailscale.com/kb/1596/secure-node-state-storage for more details.)

  Specifically, the following files will be removed.
"#);

    #[cfg(target_os = "windows")]
    println!(r#"
    - C:\ProgramData\Tailscale
    - C:\Users\%USERNAME%\AppData\Local\Tailscale
"#);

    #[cfg(target_os = "linux")]
    println!(r#"
    - /var/lib/tailscale/tailscaled.state
"#);

    println!(r#"
  Press Y to agree and continue, or N to cancel.

  "#);
}

pub fn get_user_confirmation() -> bool {
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    matches!(input.as_str(), "y" | "yes")
}
