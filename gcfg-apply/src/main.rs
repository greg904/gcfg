use std::{fs::File, path::Path, io::{self, Write}, os::unix::prelude::PermissionsExt};

fn write_file(path: impl AsRef<Path>, contents: &[u8], mode: u32) -> io::Result<()> {
    let mut f = File::options()
        .truncate(true)
        .write(true)
        .open(path)?;
    f.write_all(contents)?;
    let mut p = f.metadata()?.permissions();
    p.set_mode(mode);
    f.set_permissions(p)?;
    Ok(())
}

fn main() -> io::Result<()> {
    write_file("/etc/iwd/main.conf", include_bytes!("../../files/iwd.conf"), 0o644)?;
    write_file("/usr/local/bin/firefox-sandbox", include_bytes!("../../files/scripts/firefox-sandbox"), 0o755)?;
    write_file("/usr/local/bin/vps-port-knock", include_bytes!("../../files/scripts/vps-port-knock"), 0o755)?;
    write_file("/usr/local/bin/vps-backup", include_bytes!("../../files/scripts/vps-backup"), 0o755)?;
    Ok(())
}
