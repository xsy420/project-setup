use strum_macros::{Display, EnumIter};
#[derive(Debug, Clone, Copy, Default, EnumIter, Display, PartialEq)]
pub(crate) enum Language {
    #[default]
    Java,
    Python,
    C,
    Cpp,
    Kotlin,
    Rust,
}

impl Language {
    pub(crate) fn versions(self) -> Vec<String> {
        match self {
            Self::Java => vec!["23", "21", "17", "11", "8"],
            Self::C => vec!["90", "99", "11", "17", "23"],
            Self::Cpp => vec!["98", "03", "11", "14", "17", "20", "23", "26"],
            Self::Python => vec!["3.13", "3.12", "3.11", "3.10", "3.9", "3.8"],
            Self::Kotlin => vec!["2.1.20"],
            Self::Rust => vec!["1.86.0"],
        }
        .iter()
        .map(ToString::to_string)
        .collect()
    }
}
