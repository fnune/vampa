use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;

pub trait Host {
    fn read_source(&self, path: &str) -> io::Result<String>;
    fn write_object(&self, path: &str, bytes: &[u8]) -> io::Result<()>;
}

pub struct SystemHost;

impl Host for SystemHost {
    fn read_source(&self, path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn write_object(&self, path: &str, bytes: &[u8]) -> io::Result<()> {
        fs::write(path, bytes)?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o770))
    }
}
