use anyhow::Error;
#[cfg(feature = "reqwest")]
use reqwest::blocking::Client;
use std::path::PathBuf;
#[cfg(not(feature = "reqwest"))]
use std::process::Command;
use strum_macros::Display;
#[derive(Display, PartialEq)]
#[allow(clippy::upper_case_acronyms, dead_code)]
pub(crate) enum RequestMethod {
    GET,
    POST,
}
#[cfg(not(feature = "reqwest"))]
pub(crate) fn download_file(
    url: &str,
    method: &RequestMethod,
    params: &[(&str, String)],
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
        use std::process::Stdio;
        let output = Command::new("curl")
            .arg("--silent")
            .arg("--show-error")
            .arg("-X")
            .arg(method.to_string())
            .arg("--data")
            .arg(&form_data)
            .arg("-o")
            .arg(output)
            .arg(url)
            .stderr(Stdio::piped())
            .output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(Error::msg(unsafe {
                        String::from_utf8_unchecked(output.stderr)
                    }))
                }
            }
            Err(error) => Err(Error::new(error)),
        }
    }
    // 其次尝试 wget
    else if Command::new("wget").arg("--version").output().is_ok() {
        let mut command = Command::new("wget");
        command.arg("--quiet").arg("-O").arg(output);
        if *method == RequestMethod::POST {
            // POST 请求
            command.arg("--post-data").arg(&form_data);
        }
        // GET 请求不需要额外参数
        match command.arg(url).output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(Error::msg(unsafe {
                        String::from_utf8_unchecked(output.stderr)
                    }))
                }
            }
            Err(error) => Err(Error::new(error)),
        }
    }
    // 没有可用的下载工具时报错
    else {
        Err(Error::msg("Neither curl nor wget found in system"))
    }
}
#[cfg(feature = "reqwest")]
pub(crate) fn download_file(
    url: &str,
    method: &RequestMethod,
    params: &[(&str, String)],
    output: &PathBuf,
) -> Result<(), Error> {
    let response = match method {
        RequestMethod::GET => Client::new().get(url).query(params).send()?,
        RequestMethod::POST => Client::new().post(url).form(params).send()?,
    };
    let content = response.bytes()?;
    std::fs::write(output, content)?;
    Ok(())
}
