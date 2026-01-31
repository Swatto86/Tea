# Appendix A: Command Reference

> *Quick reference for all commands used throughout this book.*

---

## Node.js / npm

### Package Management

```bash
# Install all dependencies
npm install

# Install a specific package
npm install package-name

# Install as dev dependency
npm install --save-dev package-name

# Remove a package
npm uninstall package-name

# Update packages
npm update
```

### Scripts (from package.json)

```bash
# Start Vite development server
npm run dev

# Build frontend for production
npm run build

# Preview production build
npm run preview

# Run tests
npm test

# Run tests with UI
npm run test:ui

# Run tests once (no watch)
npm run test:run

# Run tests with coverage
npm run test:coverage
```

---

## Cargo (Rust)

### Project Management

```bash
# Create new project
cargo new project-name

# Build (debug)
cargo build

# Build (release/optimized)
cargo build --release

# Run the project
cargo run

# Run with release profile
cargo run --release

# Check for errors without building
cargo check

# Clean build artifacts
cargo clean
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Dependencies

```bash
# Update Cargo.lock
cargo update

# Update specific dependency
cargo update package-name

# Add dependency (requires cargo-edit)
cargo add package-name

# Remove dependency (requires cargo-edit)
cargo rm package-name
```

### Documentation

```bash
# Build and open documentation
cargo doc --open

# Build docs including dependencies
cargo doc --document-private-items
```

---

## Tauri CLI

### Development

```bash
# Start development mode (hot reload)
cargo tauri dev

# Start with specific features
cargo tauri dev --features feature-name

# Start targeting specific platform
cargo tauri dev --target x86_64-pc-windows-msvc
```

### Building

```bash
# Build for production
cargo tauri build

# Build with debug symbols
cargo tauri build --debug

# Build for specific target
cargo tauri build --target x86_64-pc-windows-msvc

# Build specific bundle format
cargo tauri build --bundles msi,nsis
```

### Utilities

```bash
# Generate icons from source image
cargo tauri icon path/to/icon.png

# Initialize Tauri in existing project
cargo tauri init

# Show Tauri info
cargo tauri info

# Update Tauri dependencies
cargo tauri update
```

---

## Git

### Basic Operations

```bash
# Clone repository
git clone https://github.com/user/repo.git

# Check status
git status

# Add files to staging
git add .
git add specific-file.rs

# Commit changes
git commit -m "Commit message"

# Push to remote
git push origin main
```

### Tagging and Releases

```bash
# Create tag
git tag v2.0.0

# Create annotated tag
git tag -a v2.0.0 -m "Version 2.0.0"

# Push tag
git push origin v2.0.0

# Push all tags
git push origin --tags

# List tags
git tag -l

# Delete tag locally
git tag -d v2.0.0

# Delete tag remotely
git push origin :refs/tags/v2.0.0
```

---

## Platform-Specific

### Windows (PowerShell)

```powershell
# Check file hash
Get-FileHash .\file.exe -Algorithm SHA256

# Check registry key (autostart)
reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v Tea

# Environment variables
$env:RUST_LOG = "debug"
$env:RUST_BACKTRACE = "1"

# List processes
Get-Process | Where-Object {$_.ProcessName -like "*tea*"}

# Kill process
Stop-Process -Name tea
```

### macOS / Linux (Bash)

```bash
# Check file hash
sha256sum file.dmg          # Linux
shasum -a 256 file.dmg      # macOS

# Check autostart (macOS)
ls ~/Library/LaunchAgents/ | grep tea
cat ~/Library/LaunchAgents/com.tea.app.plist

# Check autostart (Linux)
ls ~/.config/autostart/ | grep tea
cat ~/.config/autostart/tea.desktop

# Environment variables
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Find process
ps aux | grep tea

# Kill process
killall tea
# or
pkill tea
```

### Code Signing (macOS)

```bash
# Sign application
codesign --sign "Developer ID Application: Name" \
  --options runtime \
  Tea.app

# Verify signature
codesign --verify --verbose Tea.app

# Notarize
xcrun notarytool submit Tea.dmg \
  --apple-id your@email.com \
  --password app-password \
  --team-id TEAM_ID \
  --wait

# Staple notarization
xcrun stapler staple Tea.dmg

# Clear quarantine flag
xattr -cr /Applications/Tea.app
```

---

## Vitest

### Running Tests

```bash
# Run all tests
npx vitest

# Run once (no watch)
npx vitest run

# Run specific file
npx vitest frontend.test.ts

# Run matching pattern
npx vitest -t "should have heading"

# Run with coverage
npx vitest --coverage

# Run with UI
npx vitest --ui

# Watch mode (default)
npx vitest watch
```

---

## Rustup (Toolchain Management)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update

# Install specific toolchain
rustup install nightly

# Set default toolchain
rustup default stable

# Add compilation target
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu

# List installed targets
rustup target list --installed

# Show current toolchain
rustup show
```

---

## Quick Reference Table

| Task | Command |
|------|---------|
| Start development | `cargo tauri dev` |
| Build for release | `cargo tauri build` |
| Run tests (JS) | `npm test` |
| Run tests (Rust) | `cargo test` |
| Check Rust errors | `cargo check` |
| Generate icons | `cargo tauri icon icon.png` |
| Install npm deps | `npm install` |
| Update Rust | `rustup update` |
| Create git tag | `git tag v2.0.0` |
| Push tag | `git push origin v2.0.0` |
