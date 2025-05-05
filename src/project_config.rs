use std::{env, path::PathBuf};

use crate::{language::Language, project_type::ProjectType, vcs::Vcs};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct ProjectConfig {
    pub(crate) name: String,
    pub(crate) project_version: String,
    pub(crate) language_version: String,
    pub(crate) path: PathBuf,
    pub(crate) language: Language,
    pub(crate) project_type: ProjectType,
    // packaging: ProjectPackaging,
    pub(crate) vcs: Vcs,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            language: Language::default(),
            project_version: String::new(),
            language_version: String::new(),
            path: env::current_dir().unwrap(),
            project_type: ProjectType::default(),
            // packaging: ProjectPackaging::default(),
            vcs: Vcs::default(),
        }
    }
}
