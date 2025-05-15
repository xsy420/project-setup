use crate::v2::RadioOptionValue;
use anyhow::Result;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use project_setup::LoopableNumberedEnum;
use std::{
    io::Error,
    path::PathBuf,
    process::{Command, ExitStatus},
};
use strum_macros::{Display, EnumIter};
#[derive(
    Copy,
    Debug,
    Default,
    LoopableNumberedEnum,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Display,
    EnumIter,
    PartialEq,
)]
#[numbered_enum(loop_within = 6)]
pub(crate) enum Editor {
    #[default]
    NotNeed,
    Vim,
    VSCode,
    Neovim,
    Idea,
    Clion,
    Rustrover,
}
impl RadioOptionValue for Editor {
    fn selectable(&self) -> bool {
        match self {
            Self::NotNeed => true,
            _ => Command::new(self.exe())
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
        }
    }
}
impl Editor {
    fn exe(self) -> String {
        match self {
            Self::NotNeed => String::new(),
            Self::Neovim => "nvim".to_string(),
            Self::VSCode => "code".to_string(),
            _ => self.to_string().to_lowercase(),
        }
    }

    pub(crate) fn run(self, project_path: PathBuf, main: String) -> Result<ExitStatus, Error> {
        match self {
            Self::NotNeed => Ok(ExitStatus::default()),
            _ => Command::new(self.exe())
                .arg(main)
                .current_dir(project_path)
                .spawn()?
                .wait(),
        }
    }
}
