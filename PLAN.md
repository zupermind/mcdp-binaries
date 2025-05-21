This is a repo that is used to compile some rust CLI tool and make them available as a self-updating binary on several platforms.

We want to use GitHub for distribution. The repo will be private, but the binaries will be public.

The platforms to be supported are: Linux (x86/arm) and very recent OS X on M1.

We want to be able to sign the binaries.

## Implementation Plan

### 1. Repository Structure
- `src/` - Rust source code for the CLI tool
- `.github/workflows/` - CI/CD workflows
- `scripts/` - Helper scripts for building and releasing

### 2. CI/CD Pipeline (GitHub Actions)
- Build binaries for each platform (Linux x86/arm, macOS M1)
- Run on tag creation to automate releases
- Upload built binaries to GitHub Releases

### 3. Self-Update Mechanism
- Implement version checking against GitHub Releases API
- Add update command to download and replace binary
- Verify integrity using checksums

### 4. Release Process
- Tag-based releases (semantic versioning)
- Generate release notes automatically
- Create standardized binary naming: `[tool]-[version]-[platform]-[arch]`