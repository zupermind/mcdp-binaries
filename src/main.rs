use anyhow::{Context, Result};
use log::{error, info};
use zuper_rs_mcdp_cli::main_proc::main_go;
use std::env;

// Include the generated version file
include!(concat!(env!("OUT_DIR"), "/version.rs"));

// Global constants
const REPO_OWNER: &str = "zupermind";
const REPO_NAME: &str = "mcdp-binaries";
const BINARY_NAME: &str = "mcdp-cli";

fn main() -> Result<()> {
    env_logger::init();
    // Otherwise, run the main application
    eprintln!("{} {} ({})", BINARY_NAME, VERSION, ASSET_TARGET.unwrap_or("unknown"));
    

    let args: Vec<String> = env::args().collect();
    
    // Check if there is exactly one argument and it is "update"
    if args.len() >= 2 && args[1] == "update" {
        update_binary(false)?;
        return Ok(());
    }
    
    
    // Run the async application in the tokio runtime
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create tokio runtime")?
        .block_on(main_go())
}

fn update_binary(check_only: bool) -> Result<()> {
    let target = self_update::get_target();
    println!("Checking target-arch... {}", target);
    
    // Use build-time asset target if available, otherwise fall back to runtime mapping
    let asset_target = match ASSET_TARGET {
        Some(target) => target,
        None => {
            // Fallback mapping for development builds without MCDP_ASSET_TARGET
            match target {
                "x86_64-apple-darwin" => "macos15-amd64",
                "aarch64-apple-darwin" => "macos15-arm64",
                "x86_64-unknown-linux-gnu" => "ubuntu24-amd64",
                "aarch64-unknown-linux-gnu" => "ubuntu24-arm64",
                "x86_64-pc-windows-msvc" => "windows-amd64",
                "aarch64-pc-windows-msvc" => "windows-arm64",
                _ => target, // fallback to the original target
            }
        }
    };
    
    println!("Using asset target: {}", asset_target);
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
        .target(asset_target) // Use our build-time or mapped asset target
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
            Err(e) => {
                error!("Failed to get latest release: {}", e);
                println!("Failed to check for updates. This could be due to:");
                println!("  - Network connectivity issues");
                println!("  - GitHub API rate limiting");
                println!("  - Repository access issues");
                return Err(anyhow::anyhow!("Failed to get latest release: {}", e));
            }
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
            
            // Provide more helpful error messages
            let error_msg = format!("{}", e);
            if error_msg.contains("No asset found for target") {
                println!("Update failed: No compatible binary found for your platform.");
                println!("Looking for target: {}", asset_target);
                println!("Available platforms may include:");
                println!("  - ubuntu24-amd64, ubuntu24-arm64");
                println!("  - ubuntu22-amd64, ubuntu22-arm64"); 
                println!("  - macos15-amd64, macos15-arm64");
                println!("  - windows-amd64, windows-arm64");
                println!("Please check the latest release at: https://github.com/{}/{}/releases/latest", REPO_OWNER, REPO_NAME);
            } else if error_msg.contains("Network") || error_msg.contains("timeout") {
                println!("Update failed: Network connectivity issue.");
                println!("Please check your internet connection and try again.");
            } else if error_msg.contains("Permission") {
                println!("Update failed: Permission denied.");
                println!("Try running with elevated permissions (sudo on Unix, Run as Administrator on Windows).");
            } else {
                println!("Update failed with error: {}", e);
                println!("Please try again or download manually from:");
                println!("https://github.com/{}/{}/releases/latest", REPO_OWNER, REPO_NAME);
            }
            
            return Err(anyhow::anyhow!("Failed to update binary: {}", e));
        }
    }

    Ok(())
} 