mod common;
mod features;
mod v1;
mod v2;
use anyhow::Result;
use clap::Parser;
use ratatui::{
    Terminal,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::*,
};
use std::io::{self};
use v1::{ProjectSetupApp, run_app};
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
    let mut res = run_app(&mut terminal, &mut app);
    if let Err(err) = res {
        println!("{err:?}");
    }
    let mut appv2 = v2::Appv2::new();
    res = v2::run_app(&mut terminal, &mut appv2);
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
