# Release Process

This document explains how to create a new release with pre-built binaries for Claude Code plugin users.

## Prerequisites

Install [cross](https://github.com/cross-rs/cross) for cross-compilation:

```bash
cargo install cross
```

You also need Docker running for cross-compilation to work.

## Release Steps

### 1. Update Version

Use just to bump the version:

```bash
# For bug fixes
just release-patch "Fix description of changes"

# For new features
just release-minor "Feature description"

# For breaking changes
just release-major "Major change description"
```

This will:
- Update version in `Cargo.toml`
- Update `Cargo.lock`
- Create a git commit
- Create a git tag

### 2. Build Release Binaries

Build binaries for all supported platforms:

```bash
just build-release-binaries
```

This creates binaries in the `release-binaries/` directory for:
- `x86_64-unknown-linux-gnu` (Linux Intel/AMD)
- `aarch64-unknown-linux-gnu` (Linux ARM)
- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-pc-windows-gnu.exe` (Windows Intel/AMD)

### 3. Push to GitHub

Push the commit and tags to trigger the automated release:

```bash
git push origin main
git push origin --tags
```

### 4. GitHub Actions Builds and Releases Automatically

The `.github/workflows/release.yml` workflow will automatically:
1. Trigger when a tag matching `v*` is pushed
2. Build both `totui` and `totui-mcp` binaries for all platforms:
   - Linux (Intel/AMD): `x86_64-unknown-linux-gnu`
   - Linux (ARM): `aarch64-unknown-linux-gnu`
   - macOS (Intel): `x86_64-apple-darwin`
   - macOS (Apple Silicon): `aarch64-apple-darwin`
   - Windows (Intel/AMD): `x86_64-pc-windows-gnu.exe`
3. Upload all binaries to the GitHub release
4. Create or update the release on GitHub

You can monitor the workflow progress at:
https://github.com/grimurjonsson/to-tui/actions

Once complete, the release will be available at:
https://github.com/grimurjonsson/to-tui/releases

**Note:** You can still add release notes manually by editing the release on GitHub after the workflow completes.

### 5. Users Install/Update

When users install or update the plugin via Claude Code:

```bash
cd ~/.claude/plugins/repos/totui-mcp
bash scripts/install-binary.sh
```

The script will automatically:
- Detect their platform
- Download the correct binary from the latest GitHub release
- Install it to the correct location
- Make it executable

## Binary Naming Convention

Each release includes both the TUI binary and MCP server binary:
- **TUI binary**: `totui-{target}[.exe]`
- **MCP binary**: `totui-mcp-{target}[.exe]`

Where `{target}` is one of:
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-gnu` (with `.exe` extension)

The installation script (`scripts/install-binary.sh`) automatically downloads the correct `totui-mcp` binary for the user's platform.

## Troubleshooting

### Cross-compilation fails

Make sure Docker is running:
```bash
docker ps
```

### Binary doesn't work on target platform

Verify you're using the correct Rust target. You can list all targets with:
```bash
rustup target list
```

### Users report binary not found

Check that:
1. The binary was uploaded to the GitHub release
2. The binary name matches the expected format
3. The release is marked as "latest" on GitHub
