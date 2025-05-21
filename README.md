# MCDP Binary Distribution

This repository compiles and distributes self-updating Rust CLI binaries for multiple platforms.

## Supported Platforms

- Linux (x86_64, aarch64)
- macOS (x86_64, arm64/M1)

## Installation

### macOS and Linux

```bash
# Replace VERSION with the desired version (e.g., 0.1.0)
# Replace PLATFORM with linux/macos
# Replace ARCH with amd64/arm64

# Default values: 
REPO_OWNER="your-org"
REPO_NAME="mcdp-binaries"
BINARY_NAME="mcdp-tool"

curl -L https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/vVERSION/$BINARY_NAME-VERSION-PLATFORM-ARCH -o $BINARY_NAME
chmod +x $BINARY_NAME
```

For a more comprehensive installation script:

```bash
# You can override default values:
# REPO_OWNER=your-org REPO_NAME=mcdp-binaries BINARY_NAME=mcdp-tool VERSION=0.1.0 \
curl -L https://raw.githubusercontent.com/your-org/mcdp-binaries/main/scripts/install.sh | bash
```

## Usage

```bash
# Run the default command
./mcdp-tool

# See available commands
./mcdp-tool --help

# Example command
./mcdp-tool example --param "value"

# Check for updates
./mcdp-tool update --check-only

# Update to latest version
./mcdp-tool update
```

## Development

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/mcdp-binaries.git
   cd mcdp-binaries
   ```

2. Make changes to the code in `src/`

3. Build locally:
   ```bash
   cargo build --release
   ```

4. Test the binary:
   ```bash
   ./target/release/mcdp-tool
   ```

5. Release a new version:
   ```bash
   ./scripts/release.sh 0.1.0
   ```

## How the Self-Update Works

The binary uses GitHub Releases as the update source. When the `update` command is called:

1. It checks the latest release on GitHub
2. If a newer version is available, it downloads the appropriate binary for the current platform
3. It replaces the current binary with the new version
4. Integrity is verified using SHA256 checksums

## License

[MIT License](LICENSE) 