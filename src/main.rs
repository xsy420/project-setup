mod app;
mod create_project;
mod direction;
mod focus_input;
mod language;
mod project_config;
// mod project_packaging;
mod project_type;
mod vcs;
use std::io::{self};

use app::ProjectSetupApp;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::*};

/// A TUI to help you setup a project easily.
#[derive(Parser)]
#[command(version, about)]
struct Args {}

fn main() -> Result<()> {
    Args::parse();
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 创建应用
    let mut app = ProjectSetupApp::new();
    let res = app::run_app(&mut terminal, &mut app);

    // 清理终端
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

#[cfg(test)]
mod hello {
    #[test]
    fn world() {
        println!("hello world");
    }
}
