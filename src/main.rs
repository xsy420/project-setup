use anyhow::Result;
use clap::Parser;
use project_setup::{app::Application, args::Args};
use ratatui::{
    Terminal,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::*,
};
use std::io::{self};
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    Application::prepare_app(&mut terminal)?;
    // 创建应用
    let app = Application::new(args);
    let res = app.run(&mut terminal);
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
