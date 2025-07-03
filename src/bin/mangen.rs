use clap::CommandFactory;
use clap_mangen::Man;
use project_setup::args::Args;
use std::io::Result;
use std::path::PathBuf;
use std::{env, fs};
/// Man page can be created with:
/// `cargo run --bin project-setup-mangen`
/// in a directory specified by the environment variable `OUT_DIR`.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out_path = PathBuf::from(out_dir).join(format!("{}.1", env!("CARGO_PKG_NAME")));
    let app = Args::command();
    let man = Man::new(app);
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;
    fs::write(&out_path, buffer)?;
    println!("Man page is generated at {}", out_path.display());
    Ok(())
}
