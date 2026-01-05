use crate::app::{PreparePermit, PrepareRecv, PrepareTrait};
use crate::common::{Editor, Vcs};
use std::process::Command;
use std::sync::LazyLock;
use std::{collections::HashMap, sync::Mutex};
use strum::IntoEnumIterator;
pub(crate) trait ExecutableEnumTrait {
    fn exe(&self) -> String;
}
static EXECUTABLE_CACHE: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
pub(crate) struct Executable {}
impl PrepareTrait for Executable {
    async fn prepare(permit: &mut PreparePermit<'_>, offset: f64) -> bool {
        let cvs: Vec<String> = Self::list(false);
        let mut recv = PrepareRecv::new(offset);
        for ele in cvs {
            let res = ele.is_empty()
                || Command::new(ele.clone())
                    .arg(if ele.eq("7z") { "--help" } else { "--version" })
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);
            EXECUTABLE_CACHE.lock().unwrap().insert(ele, res);
            recv.send_ok(permit);
        }
        true
    }

    fn descs() -> Vec<String> {
        Self::list(true)
    }
}
impl Executable {
    fn list(descs: bool) -> Vec<String> {
        Vcs::iter()
            .map(|x| x.exe())
            .chain(Editor::iter().map(|x| x.exe()))
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
        *EXECUTABLE_CACHE.lock().unwrap().get(cmd).unwrap_or(&false)
    }
}
