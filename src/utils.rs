use std::process::Command;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}

pub fn bytes_to_mbytes(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}