#![feature(unix_chown)]

use std::{
    fs::File,
    io::{self, Write},
    os::unix::{fs, prelude::PermissionsExt},
    path::Path,
};

#[derive(PartialEq, Eq)]
enum Machine {
    Pc,
    Vps,
}

fn read_machine_from_hostname() -> io::Result<Machine> {
    match hostname::get()?.to_str().unwrap_or("") {
        "pc" => Ok(Machine::Pc),
        "gregdf.com" => Ok(Machine::Vps),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid hostname",
        )),
    }
}

fn write_file(path: impl AsRef<Path>, contents: &[u8], mode: u32, user: bool) -> io::Result<()> {
    let mut f = File::options()
        .truncate(true)
        .write(true)
        .create(true)
        .open(&path)?;
    f.write_all(contents)?;
    let mut p = f.metadata()?.permissions();
    p.set_mode(mode);
    f.set_permissions(p)?;
    if user {
        fs::chown(&path, Some(1000), Some(1000))?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let machine = read_machine_from_hostname()?;
    if machine == Machine::Pc {
        write_file(
            "/etc/iwd/main.conf",
            include_bytes!("../../files/iwd.conf"),
            0o644,
            false,
        )?;
        write_file(
            "/usr/local/bin/firefox-sandbox",
            include_bytes!("../../files/scripts/firefox-sandbox"),
            0o755,
            false,
        )?;
        write_file(
            "/usr/local/bin/vps-port-knock",
            include_bytes!("../../files/scripts/vps-port-knock"),
            0o755,
            false,
        )?;
        write_file(
            "/usr/local/bin/vps-backup",
            include_bytes!("../../files/scripts/vps-backup"),
            0o755,
            false,
        )?;
        write_file(
            "/home/greg/.config/alacritty/alacritty.yml",
            include_bytes!("../../files/alacritty.yml"),
            0o644,
            true,
        )?;
        write_file(
            "/home/greg/.config/sway/config",
            include_bytes!("../../files/sway"),
            0o644,
            true,
        )?;
        write_file(
            "/home/greg/.config/nvim/init.lua",
            include_bytes!("../../files/nvim/init.lua"),
            0o644,
            true,
        )?;
        write_file(
            "/home/greg/.config/nvim/UltiSnips/tex.snippets",
            include_bytes!("../../files/nvim/tex.snippets"),
            0o644,
            true,
        )?;
        write_file(
            "/home/greg/.ssh/config",
            include_bytes!("../../files/ssh"),
            0o644,
            true,
        )?;
    }
    write_file(
        "/home/greg/.zshrc",
        include_bytes!("../../files/zshrc"),
        0o644,
        true,
    )?;
    Ok(())
}
