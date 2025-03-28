use std::io;
use std::path::PathBuf;

pub fn print_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("AESExtractor v{}\n", version);
}
pub fn path_buf_ends_with(path: &PathBuf, ext: &str) -> bool {
    path.to_string_lossy().ends_with(ext)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

pub fn wait_for_exit() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(())
}