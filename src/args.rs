use crate::common::ProjectType;
use clap::Parser;
/// A TUI to help you setup a project easily.
#[derive(Parser, Clone, Copy)]
#[command(version, about)]
pub struct Args {
    /// the only project type to create (skips project selection)
    #[arg(short, long)]
    pub(crate) project_type: Option<ProjectType>,
}
