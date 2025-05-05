use anyhow::Error;
use std::{ffi::OsStr, path::PathBuf, process::Command};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Default, Display, Clone, EnumIter, PartialEq)]
pub(crate) enum Vcs {
    #[default]
    NotNeed,
    Git,
    Svn,
}

impl Vcs {
    pub(crate) fn is_available(&self) -> bool {
        match self {
            Self::NotNeed => true,
            Self::Git | Self::Svn => Command::new(self)
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
        }
    }

    pub(crate) fn init_vcs_repo(&self, name: &String, path: &PathBuf) -> Result<(), Error> {
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
