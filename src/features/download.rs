use anyhow::Error;
#[cfg(feature = "reqwest")]
use reqwest::blocking::Client;
use std::path::PathBuf;
#[cfg(not(feature = "reqwest"))]
use std::process::Command;
#[cfg(not(feature = "reqwest"))]
pub(crate) fn download_file(
    url: &str,
    params: &[(&str, &str)],
    output: &PathBuf,
) -> Result<(), Error> {
    // 构建表单参数字符串（格式：key1=value1&key2=value2）
    let form_data = params
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");
    // 优先尝试 curl
    if Command::new("curl").arg("--version").output().is_ok() {
        Command::new("curl")
            .arg("--silent")
            .arg("-X")
            .arg("POST")
            .arg("--data")
            .arg(&form_data)
            .arg("-o")
            .arg(output)
            .arg(url)
            .status()?;
    }
    // 其次尝试 wget
    else if Command::new("wget").arg("--version").output().is_ok() {
        Command::new("wget")
            .arg("--quiet")
            .arg("--post-data")
            .arg(&form_data)
            .arg("-O")
            .arg(output)
            .arg(url)
            .status()?;
    }
    // 没有可用的下载工具时报错
    else {
        return Err(Error::msg("Neither curl nor wget found in system"));
    }
    Ok(())
}
#[cfg(feature = "reqwest")]
pub(crate) fn download_file(
    url: &str,
    params: &[(&str, &str)],
    output: &PathBuf,
) -> Result<(), Error> {
    let response = Client::new().post(url).form(&params).send()?;
    let content = response.bytes()?;
    std::fs::write(output, content)?;
    Ok(())
}
