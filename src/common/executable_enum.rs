use crate::app::{PrepareRecv, PrepareTrait};
use crate::common::{Editor, Vcs};
use std::process::Command;
use std::sync::LazyLock;
use std::{collections::HashMap, sync::Mutex};
use strum::IntoEnumIterator;
use tokio::sync::mpsc;
pub(crate) trait ExecutableEnumTrait {
    fn exe(&self) -> String;
}
pub static EXECUTABLE_ENUM_CACHE: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
pub(crate) struct ExecutableEnum {}
impl PrepareTrait for ExecutableEnum {
    async fn prepare(permit: &mut mpsc::PermitIterator<'_, PrepareRecv>, offset: f64) {
        let cvs: Vec<String> = Self::list(false);
        let mut recv = PrepareRecv::new(offset);
        for ele in cvs {
            let res = ele.is_empty()
                || Command::new(ele.clone())
                    .arg("--version")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);
            EXECUTABLE_ENUM_CACHE.lock().unwrap().insert(ele, res);
            recv.send(permit);
            recv.next_step();
        }
    }

    fn descs() -> Vec<String> {
        Self::list(true)
    }
}
impl ExecutableEnum {
    fn list(descs: bool) -> Vec<String> {
        Vcs::iter()
            .map(|x| x.exe())
            .chain(Editor::iter().map(|x| x.exe()))
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
}
