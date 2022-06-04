#![feature(unix_chown)]

use std::{
    fs::File,
    io::{self, Write},
    os::unix::prelude::{CommandExt, PermissionsExt},
    path::Path,
    process::Command,
    time::{Duration, SystemTime},
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

fn clone_or_update_git_repo_as_user(origin: &str, path: impl AsRef<Path>) -> io::Result<()> {
    // If the last pull was done recently, then we consider the
    // repository to be up-to-date. This makes the command faster
    // and prevents hammering the remote repository.
    match path.as_ref().metadata() {
        Ok(m) => {
            if !m.is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "git repository destination is not a directory",
                ));
            }
            let dot_git = path.as_ref().join(".git");
            let timeout = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
            if dot_git
                .join("FETCH_HEAD")
                .metadata()
                .and_then(|m| m.modified())
                .map(|m| m > timeout)
                .unwrap_or(false)
                || dot_git
                    .join("HEAD")
                    .metadata()
                    .and_then(|m| m.modified())
                    .map(|m| m > timeout)
                    .unwrap_or(false)
            {
                return Ok(());
            }
            let code = Command::new("git")
                .arg("-C")
                .arg(path.as_ref().to_str().unwrap())
                .arg("pull")
                .uid(1000)
                .gid(1000)
                .spawn()?
                .wait()?;
            if !code.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Git returned with unsuccessful exit code: {:?}",
                        code.code()
                    ),
                ));
            }
            return Ok(());
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    let code = Command::new("git")
        .arg("clone")
        .arg("--")
        .arg(origin)
        .arg(path.as_ref().to_str().unwrap())
        .uid(1000)
        .gid(1000)
        .spawn()?
        .wait()?;
    if !code.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Git returned with unsuccessful exit code: {:?}",
                code.code()
            ),
        ));
    }
    Ok(())
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
        std::os::unix::fs::chown(&path, Some(1000), Some(1000))?;
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
        clone_or_update_git_repo_as_user(
            "https://github.com/lervag/vimtex.git",
            "/home/greg/.local/share/nvim/site/pack/my-plugins/start/vimtex",
        )?;
        clone_or_update_git_repo_as_user(
            "https://github.com/neovim/nvim-lspconfig.git",
            "/home/greg/.local/share/nvim/site/pack/my-plugins/start/nvim-lspconfig",
        )?;
        clone_or_update_git_repo_as_user(
            "https://github.com/quangnguyen30192/cmp-nvim-ultisnips.git",
            "/home/greg/.local/share/nvim/site/pack/my-plugins/start/cmp-nvim-ultisnips",
        )?;
        clone_or_update_git_repo_as_user(
            "https://github.com/SirVer/ultisnips.git",
            "/home/greg/.local/share/nvim/site/pack/my-plugins/start/ultisnips",
        )?;
        clone_or_update_git_repo_as_user(
            "https://github.com/hrsh7th/cmp-nvim-lsp.git",
            "/home/greg/.local/share/nvim/site/pack/my-plugins/start/cmp-nvim-lsp",
        )?;
        clone_or_update_git_repo_as_user(
            "https://github.com/hrsh7th/nvim-cmp.git",
            "/home/greg/.local/share/nvim/site/pack/my-plugins/start/nvim-cmp",
        )?;
    }
    write_file(
        "/home/greg/.zshrc",
        include_bytes!("../../files/zshrc"),
        0o644,
        true,
    )?;
    clone_or_update_git_repo_as_user(
        "https://github.com/zsh-users/zsh-autosuggestions.git",
        "/home/greg/.local/share/zsh-plugins/zsh-autosuggestions",
    )?;
    clone_or_update_git_repo_as_user(
        "https://github.com/zsh-users/zsh-history-substring-search.git",
        "/home/greg/.local/share/zsh-plugins/zsh-history-substring-search",
    )?;
    clone_or_update_git_repo_as_user(
        "https://github.com/airblade/vim-rooter.git",
        "/home/greg/.local/share/nvim/site/pack/my-plugins/start/vim-rooter",
    )?;
    clone_or_update_git_repo_as_user(
        "https://github.com/editorconfig/editorconfig-vim.git",
        "/home/greg/.local/share/nvim/site/pack/my-plugins/start/editorconfig-vim",
    )?;
    clone_or_update_git_repo_as_user(
        "https://github.com/itchyny/lightline.vim",
        "/home/greg/.local/share/nvim/site/pack/my-plugins/start/lightline",
    )?;
    Ok(())
}
