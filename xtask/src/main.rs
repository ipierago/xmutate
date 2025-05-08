use clap::{Parser, Subcommand};
use std::process::{Command};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "xtask")]
struct Xtask {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    TestMutators,
    Dist,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xtask = Xtask::parse();

    match xtask.command {
        Commands::TestMutators => {
            let status = Command::new("cargo")
                .args(&["test", "-p", "xmutate"])
                .status()?;

            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        },
        Commands::Dist => dist()?,
    }

    Ok(())
}

fn dist() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args(["build", "--release", "-p", "xmutate"])
        .status()?;

    if !status.success() {
        return Err("Build failed".into());
    }

    let target = Path::new("target/release/xmutate");
    let dist_dir = Path::new("dist");

    fs::create_dir_all(dist_dir)?;
    fs::copy(target, dist_dir.join("xmutate"))?;

    println!("✅ Copied binary to dist/xmutate");
    Ok(())
}