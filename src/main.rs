use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{error, info};
use self_update::cargo_crate_version;
use zuper_rs_mcdp_cli::main_proc::main_go;

// Global constants
const REPO_OWNER: &str = "zupermind";
const REPO_NAME: &str = "mcdp-binaries";
const BINARY_NAME: &str = "mcdp-tool";

#[derive(Parser)]
#[command(name = BINARY_NAME)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Update the tool to the latest version
    Update {
        /// Check if an update is available without installing it
        #[arg(short, long)]
        check_only: bool,
    },
    
    /// Example command
    Example {
        /// An example parameter
        #[arg(short, long)]
        param: Option<String>,
    },
}


#[tokio::main]
async fn main() -> Result<()> {

    //
    // // only check if there is exactly one argument that is update
    // if args.len() == 2 && args[1] == "update" {
    //     update_binary(false)?;
    //     return Ok(());
    // }
    env_logger::init();

    let a = zuper_rs_mcdp_cli::main_proc::main_go();
    
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Update { check_only }) => {
            update_binary(*check_only)?;
        }
        Some(Commands::Example { param }) => {
            println!("Running example command with param: {:?}", param);
        }
        None => {
            println!("Running default action...");
            println!("Current version: {}", cargo_crate_version!());
        }
    }

    Ok(())
}

fn update_binary(check_only: bool) -> Result<()> {
    let target = self_update::get_target();
    println!("Checking target-arch... {}", target);
    
    // Map Rust target triple to our asset naming convention
    let asset_target = match target  {
        "x86_64-apple-darwin" => "macos-amd64",
        "aarch64-apple-darwin" => "macos-arm64",
        "x86_64-unknown-linux-gnu" => "linux-amd64",
        "aarch64-unknown-linux-gnu" => "linux-arm64",
        "x86_64-pc-windows-msvc" => "windows-amd64",
        "aarch64-pc-windows-msvc" => "windows-arm64",
        _ => target, // fallback to the original target
    };
    //
    let current_version = cargo_crate_version!();
    println!("Checking current version... v{}", current_version);
    println!("Checking latest released version... ");

    let binary_name =  if asset_target.starts_with("windows") {
        format!("{}.exe", BINARY_NAME)
    } else {
        BINARY_NAME.to_string()
    };

    let status = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(&binary_name)
        .target(asset_target) // Use our mapped asset target
        .show_download_progress(true)
        .current_version(current_version)
        .build()
        .context("Failed to build updater")?;

    if check_only {
        match status.get_latest_release() {
            Ok(release) => {
                if current_version == release.version {
                    println!("Already up to date at version {}", current_version);
                } else {
                    println!("Update available: {}", release.version);
                }
            }
            Err(e) => error!("Failed to get latest release: {}", e),
        }
        return Ok(());
    }

    match status.update() {
        Ok(update_result) => {
            if update_result.updated() {
                info!("Updated successfully to {}", update_result.version());
                println!("Updated successfully to {}", update_result.version());
            } else {
                println!("Already up to date at version {}", current_version);
            }
        }
        Err(e) => {
            error!("Failed to update binary: {}", e);
            return Err(anyhow::anyhow!("Failed to update binary: {}", e));
        }
    }

    Ok(())
} 