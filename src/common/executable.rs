use crate::app::{PreparePermit, PrepareRecv, PrepareTrait};
use std::collections::HashMap;
use std::process::Command;
use std::sync::OnceLock;
use strum::IntoEnumIterator;
pub(crate) trait ExecutableEnumTrait {
    fn exe(&self) -> String;
}
static EXECUTABLE_CACHE: OnceLock<HashMap<String, bool>> = OnceLock::new();
pub(crate) struct Executable {}
impl PrepareTrait for Executable {
    async fn prepare(permit: &mut PreparePermit<'_>, offset: f64) -> bool {
        let cvs: Vec<String> = Self::list(false);
        let mut recv = PrepareRecv::new(offset);
        let mut cache: HashMap<String, bool> = HashMap::new();
        for ele in cvs {
            let res = ele.is_empty()
                || Command::new(ele.clone())
                    .arg(if ele.eq("7z") { "--help" } else { "--version" })
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);
            cache.insert(ele, res);
            recv.send_ok(permit);
        }
        let _ = EXECUTABLE_CACHE.set(cache);
        true
    }

    fn descs() -> Vec<String> {
        Self::list(true)
    }
}
impl Executable {
    fn list(descs: bool) -> Vec<String> {
        super::Vcs::iter()
            .map(|x| x.exe())
            .chain(super::Editor::iter().map(|x| x.exe()))
            .chain(["curl", "wget", "unzip", "tar", "7z"].map(ToString::to_string))
            .map(|s| {
                if descs {
                    format!(
                        "checking {s}{}executable...",
                        if s.is_empty() { "" } else { " " }
                    )
                } else {
                    s
                }
            })
            .collect()
    }

    pub(crate) fn executable(cmd: &str) -> bool {
        *EXECUTABLE_CACHE.get().unwrap().get(cmd).unwrap_or(&false)
    }
}
