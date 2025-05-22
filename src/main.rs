use anyhow::{Context, Result};
use log::{error, info};
use self_update::cargo_crate_version;
use zuper_rs_mcdp_cli::main_proc::main_go;
use std::env;
use std::process::Command;

// Include the generated version file
include!(concat!(env!("OUT_DIR"), "/version.rs"));

// Global constants
const REPO_OWNER: &str = "zupermind";
const REPO_NAME: &str = "mcdp-binaries";
const BINARY_NAME: &str = "mcdp-tool";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    // Check if there is exactly one argument and it is "update"
    if args.len() == 2 && args[1] == "update" {
        update_binary(false)?;
        return Ok(());
    }
    
    // Otherwise, run the main application
    println!("Current version: {}", VERSION);
    main_go().await
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
    
    println!("Checking current version... {}", VERSION);
    println!("Checking latest released version... ");

    let binary_name = if asset_target.starts_with("windows") {
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
        .current_version(VERSION)
        .build()
        .context("Failed to build updater")?;

    if check_only {
        match status.get_latest_release() {
            Ok(release) => {
                if VERSION == release.version {
                    println!("Already up to date at version {}", VERSION);
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
                println!("Already up to date at version {}", VERSION);
            }
        }
        Err(e) => {
            error!("Failed to update binary: {}", e);
            return Err(anyhow::anyhow!("Failed to update binary: {}", e));
        }
    }

    Ok(())
} 