use crate::v2::RadioOptionValue;
use anyhow::Error;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup_derive::LoopableNumberedEnum;
use std::{ffi::OsStr, path::PathBuf, process::Command};
use strum_macros::{Display, EnumIter};
#[derive(
    Copy,
    Debug,
    Default,
    Display,
    Clone,
    EnumIter,
    PartialEq,
    FromPrimitive,
    ToPrimitive,
    LoopableNumberedEnum,
)]
#[numbered_enum(loop_within = 3)]
pub(crate) enum Vcs {
    #[default]
    NotNeed,
    Git,
    Svn,
}
impl Vcs {
    pub(crate) fn init_vcs_repo(self, name: &String, path: &PathBuf) -> Result<(), Error> {
        match self {
            Vcs::Git => Command::new("git")
                .arg("init")
                .current_dir(path.join(name))
                .output()
                .map_err(|e| Error::msg(format!("Failed to execute git init: {e}"))),
            Vcs::Svn => Command::new("svnadmin")
                .arg("create")
                .arg(name)
                .current_dir(path)
                .output()
                .map_err(|e| Error::msg(format!("Failed to execute svnadmin create: {e}"))),
            Vcs::NotNeed => {
                return Ok(());
            }
        }?; // The ? operator will early return if there's an error
        Ok(())
    }
}
impl AsRef<OsStr> for Vcs {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(match self {
            Self::Git => "git",
            Self::Svn => "svn",
            Self::NotNeed => "",
        })
    }
}
impl RadioOptionValue for Vcs {
    fn selectable(&self) -> bool {
        match self {
            Self::NotNeed => true,
            Self::Git | Self::Svn => Command::new(self)
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
        }
    }
}
