# Distribution Guide

This guide explains how to distribute `solana-txn-tui` so users can install it via `cargo install` or `brew install`.

## Prerequisites

- GitHub account
- crates.io account (for cargo publishing)
- Git installed locally

---

## Part 1: Publish to crates.io (cargo install)

### Step 1: Prepare Your Package

Ensure your `Cargo.toml` has all required metadata:

```toml
[package]
name = "solana-txn-tui"
version = "0.1.0"  # Update this for new releases
authors = ["Your Name <your.email@example.com>"]
description = "A comprehensive TUI for exploring Solana transactions and accounts"
license = "MIT"
repository = "https://github.com/YOUR_USERNAME/solana-txn-tui"
homepage = "https://github.com/YOUR_USERNAME/solana-txn-tui"
readme = "README.md"
keywords = ["solana", "blockchain", "tui", "terminal", "crypto"]
categories = ["command-line-utilities"]
```

### Step 2: Create a crates.io Account

1. Go to https://crates.io/
2. Log in with your GitHub account
3. Go to Account Settings: https://crates.io/settings/profile
4. Verify your email address
5. Create an API token: https://crates.io/settings/tokens
6. **Copy the token immediately** (you can't see it again!)

### Step 3: Login with Cargo

```bash
cargo login
```

Paste your API token when prompted.

### Step 4: Prepare for Publishing

1. **Update version number** in `Cargo.toml` if needed
2. **Update CHANGELOG.md** (create if doesn't exist):
   ```markdown
   # Changelog

   ## [0.1.0] - 2026-02-02
   ### Added
   - Initial release
   - Transaction viewing support (Overview, Accounts, Instructions, Logs)
   - Account viewing support
   - Multi-network support (Mainnet/Devnet/Testnet)
   - Detailed instruction view
   - Token transfer parsing
   ```

3. **Commit all changes**:
   ```bash
   git add .
   git commit -m "Prepare for v0.1.0 release"
   ```

4. **Create a git tag**:
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```

### Step 5: Test Before Publishing

```bash
# Run tests
cargo test

# Check for warnings
cargo clippy

# Dry run publish (verifies everything)
cargo publish --dry-run

# Check what's included in the package
cargo package --list
```

### Step 6: Publish

```bash
cargo publish
```

Your package is now live on crates.io! Users can install it with:

```bash
cargo install solana-txn-tui
```

### Step 7: Verify Installation

```bash
# Test from a clean environment
cargo install solana-txn-tui
solana-txn-tui
```

---

## Part 2: Create Homebrew Formula (brew install)

### Option A: Create Your Own Tap (Recommended)

This is faster than submitting to homebrew-core and gives you full control.

#### Step 1: Create a Homebrew Tap Repository

1. Create a new GitHub repository named: `homebrew-solana-txn-tui`
2. Make it public
3. Initialize with a README

#### Step 2: Create the Formula

In your new tap repository, create a file: `Formula/solana-txn-tui.rb`

```ruby
class SolanaTxnTui < Formula
  desc "Comprehensive TUI for exploring Solana transactions and accounts"
  homepage "https://github.com/YOUR_USERNAME/solana-txn-tui"
  url "https://github.com/YOUR_USERNAME/solana-txn-tui/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"
  head "https://github.com/YOUR_USERNAME/solana-txn-tui.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "solana-txn-tui", shell_output("#{bin}/solana-txn-tui --version")
  end
end
```

#### Step 3: Calculate the SHA256

Get the SHA256 hash of your release tarball:

```bash
# Download the tarball
curl -L -o solana-txn-tui-0.1.0.tar.gz \
  https://github.com/YOUR_USERNAME/solana-txn-tui/archive/refs/tags/v0.1.0.tar.gz

# Calculate SHA256
shasum -a 256 solana-txn-tui-0.1.0.tar.gz
```

Copy the hash and update the formula.

#### Step 4: Commit and Push

```bash
git add Formula/solana-txn-tui.rb
git commit -m "Add solana-txn-tui formula v0.1.0"
git push origin main
```

#### Step 5: Users Can Now Install

```bash
brew tap YOUR_USERNAME/solana-txn-tui
brew install solana-txn-tui
```

### Option B: Submit to Homebrew Core (Official)

This makes your package available to all Homebrew users without needing a tap.

#### Step 1: Fork Homebrew Core

```bash
# Fork https://github.com/Homebrew/homebrew-core on GitHub

# Clone your fork
git clone https://github.com/YOUR_USERNAME/homebrew-core.git
cd homebrew-core
```

#### Step 2: Create Your Formula

```bash
# Use the template
brew create --set-name solana-txn-tui \
  https://github.com/YOUR_USERNAME/solana-txn-tui/archive/refs/tags/v0.1.0.tar.gz
```

This opens your editor. Replace the content with:

```ruby
class SolanaTxnTui < Formula
  desc "Comprehensive TUI for exploring Solana transactions and accounts"
  homepage "https://github.com/YOUR_USERNAME/solana-txn-tui"
  url "https://github.com/YOUR_USERNAME/solana-txn-tui/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "YOUR_SHA256_HERE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "solana-txn-tui", shell_output("#{bin}/solana-txn-tui --version")
  end
end
```

#### Step 3: Test Your Formula

```bash
# Install locally
brew install --build-from-source ./solana-txn-tui.rb

# Run tests
brew test ./solana-txn-tui.rb

# Audit the formula
brew audit --strict --new --online ./solana-txn-tui.rb
```

#### Step 4: Submit Pull Request

```bash
git checkout -b add-solana-txn-tui
git add Formula/s/solana-txn-tui.rb
git commit -m "solana-txn-tui 0.1.0 (new formula)"
git push origin add-solana-txn-tui
```

Then create a pull request on GitHub to `Homebrew/homebrew-core`.

---

## Part 3: Automated Releases (Optional but Recommended)

### Using GitHub Actions

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  publish-crates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - uses: actions-rs/cargo@v1
        with:
          command: publish
          args: --token ${{ secrets.CRATES_IO_TOKEN }}

  build-binaries:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
          override: true
      - uses: actions-rs/cargo@v1
        with:
          use-cross: true
          command: build
          args: --release --target ${{ matrix.target }}
      - name: Upload binary
        uses: actions/upload-artifact@v3
        with:
          name: solana-txn-tui-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/solana-txn-tui
```

Add your crates.io token as a GitHub secret named `CRATES_IO_TOKEN`.

---

## Quick Reference

### Publishing Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run `cargo test`
- [ ] Run `cargo clippy`
- [ ] Run `cargo publish --dry-run`
- [ ] Commit changes
- [ ] Create and push git tag
- [ ] Run `cargo publish`
- [ ] Update Homebrew formula with new SHA256
- [ ] Test Homebrew installation
- [ ] Update README with new version

### Version Format

Use Semantic Versioning: `MAJOR.MINOR.PATCH`
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

### Commands Summary

```bash
# Crates.io
cargo login                    # Login to crates.io
cargo publish --dry-run       # Test publish
cargo publish                 # Actually publish

# Homebrew (own tap)
brew tap YOUR_USERNAME/solana-txn-tui
brew install solana-txn-tui
brew audit --strict --new --online solana-txn-tui

# Homebrew (core - for PR)
brew create --set-name solana-txn-tui <URL>
brew install --build-from-source ./solana-txn-tui.rb
brew test ./solana-txn-tui.rb
brew audit --strict --new --online ./solana-txn-tui.rb
```

---

## Troubleshooting

### crates.io Issues

**Error: "crate name already taken"**
- Choose a different name or contact the owner

**Error: "missing field"**
- Add required fields to `Cargo.toml` (license, description, etc.)

**Error: "git repository dirty"**
- Commit all changes before publishing

### Homebrew Issues

**Error: "SHA256 mismatch"**
- Recalculate SHA256 of the tarball

**Error: "no bottle available"**
- Build from source with `--build-from-source`

**Error: "Formula requires Rust"**
- Ensure `depends_on "rust" => :build` is in the formula

---

## Support

For issues with:
- **crates.io**: https://doc.rust-lang.org/cargo/reference/publishing.html
- **Homebrew**: https://docs.brew.sh/Adding-Software-to-Homebrew

## Next Steps

1. Create a crates.io account and publish
2. Create a Homebrew tap repository
3. Add the GitHub Actions workflow for automation
4. Share your package with the community!
