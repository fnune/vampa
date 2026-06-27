use std::fs;
use std::io;
use std::process::Command;

const LINKER: &str = env!(
    "VAMPA_CC",
    "VAMPA_CC is unset; build through the Nix flake (`nix build` or `nix develop`)."
);

pub trait Host {
    fn read_source(&self, path: &str) -> io::Result<String>;
    fn write_object(&self, path: &str, bytes: &[u8]) -> io::Result<()>;
    fn link(&self, object_path: &str, executable_path: &str) -> io::Result<()>;
}

pub struct SystemHost;

impl Host for SystemHost {
    fn read_source(&self, path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn write_object(&self, path: &str, bytes: &[u8]) -> io::Result<()> {
        fs::write(path, bytes)
    }

    fn link(&self, object_path: &str, executable_path: &str) -> io::Result<()> {
        let status = Command::new(LINKER)
            .args([object_path, "-o", executable_path])
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::other(format!("cc exited with status {status}")))
        }
    }
}
