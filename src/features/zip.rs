use anyhow::Error;
#[cfg(not(feature = "external_zip"))]
use std::fs::File;
#[cfg(feature = "external_zip")]
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
#[cfg(feature = "external_zip")]
use std::process::Command;
#[cfg(not(feature = "external_zip"))]
use zip::ZipArchive;
#[cfg(feature = "external_zip")]
pub(crate) fn unzip(zip_path: &PathBuf, output_dir: &PathBuf) -> Result<(), Error> {
    let zip_path = Path::new(zip_path).canonicalize()?;
    let output_dir = Path::new(output_dir).canonicalize()?;
    // 优先尝试 unzip (Linux/macOS/Windows if installed)
    if Command::new("unzip").arg("--help").output().is_ok() {
        Command::new("unzip")
            .arg("-q")
            .arg("-o")
            .arg(&zip_path)
            .arg("-d")
            .arg(&output_dir)
            .status()?;
        return Ok(());
    }
    // 其次尝试 7z (跨平台)
    if Command::new("7z").arg("--help").output().is_ok() {
        Command::new("7z")
            .arg("x")
            .arg("-y")
            .arg(format!("-o{}", output_dir.display()))
            .arg(&zip_path)
            .status()?;
        return Ok(());
    }
    // Windows 原生解压方式
    #[cfg(target_os = "windows")]
    {
        // 方法1: 使用 tar (Windows 10+ 内置)
        if let Ok(_) = Command::new("tar").arg("--version").output() {
            Command::new("tar")
                .arg("-xf")
                .arg(&zip_path)
                .arg("-C")
                .arg(&output_dir)
                .status()?;
            return Ok(());
        }
        // 方法2: 使用 PowerShell 的 Expand-Archive
        let ps_script = format!(
            "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
            zip_path.display(),
            output_dir.display()
        );
        let status = Command::new("powershell")
            .args(&["-Command", &ps_script])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .status()?;
        if status.success() {
            return Ok(());
        }
    }
    Err(Error::msg(
        "No available zip extraction tool found (tried: unzip, 7z, tar, PowerShell)",
    ))
}
#[cfg(not(feature = "external_zip"))]
pub(crate) fn unzip(zip_path: &PathBuf, output_dir: &PathBuf) -> Result<(), Error> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    for i in 0 .. archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = Path::new(output_dir).join(file.mangled_name());
        if file.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out_file = File::create(&out_path)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }
    Ok(())
}
