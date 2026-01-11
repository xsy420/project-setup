use anyhow::Result;
use clap::Parser;
use project_setup::{
    app::{Application, PrepareApplication},
    args::Args,
};
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let res = ratatui::run(|terminal| {
        PrepareApplication::default().run(terminal)?;
        Application::new(args).run(terminal)
    });
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
