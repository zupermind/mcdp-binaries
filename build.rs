use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun this script if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/main");
    println!("cargo:rerun-if-changed=.git/refs/tags");

    // Get the output directory where we'll write our generated file
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("version.rs");

    // Try to get version from git tag
    let version = get_git_version().unwrap_or_else(|| {
        // Fallback to Cargo.toml version
        env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string())
    });

    // Get asset target from environment variable, with fallback to None
    let asset_target = env::var("MCDP_ASSET_TARGET").ok();

    // Write the version and asset target info to a file that will be included at compile time
    let mut content = format!("pub const VERSION: &str = \"{}\";\n", version);
    
    if let Some(target) = asset_target {
        content.push_str(&format!("pub const ASSET_TARGET: Option<&str> = Some(\"{}\");\n", target));
    } else {
        content.push_str("pub const ASSET_TARGET: Option<&str> = None;\n");
    }

    fs::write(&dest_path, content).expect("Failed to write version file");
}

fn get_git_version() -> Option<String> {
    // Try to get version from VERSION environment variable first
    if let Ok(version) = env::var("VERSION") {
        if !version.is_empty() {
            return Some(version);
        }
    }

    // Otherwise try to get it from git
    let output = Command::new("git")
        .args(["describe", "--tags", "--always"])
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| {
                let version = s.trim();
                // Remove leading 'v' if present to ensure consistent format
                if version.starts_with('v') {
                    version[1..].to_string()
                } else {
                    version.to_string()
                }
            })
    } else {
        None
    }
} 