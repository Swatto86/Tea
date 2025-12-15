# Building Tea from Source

## Prerequisites

### All Platforms
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (LTS version)

### Linux
- Build essentials
  ```bash
  # Ubuntu/Debian
  sudo apt-get install build-essential libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev xdotool
  
  # Fedora
  sudo dnf install webkit2gtk4.0-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel xdotool
  
  # Arch/Manjaro
  sudo pacman -S webkit2gtk gtk3 libappindicator-gtk3 librsvg xdotool base-devel
  ```

### Windows
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- WebView2 Runtime (usually pre-installed on Windows 10/11)

### macOS
- Xcode Command Line Tools
  ```bash
  xcode-select --install
  ```

## Building

1. Clone the repository:
```bash
git clone https://github.com/yourusername/tea.git
cd tea
```

2. Install dependencies:
```bash
npm install
```

3. Build the application:
```bash
cargo tauri build
```

The built application will be available in:
- Linux: `src-tauri/target/release/tea`
- Windows: `src-tauri/target/release/tea.exe`
- macOS: `src-tauri/target/release/tea.app`

## Development

For development, you can use:
```bash
cargo tauri dev
```

This will start the application in development mode with hot reloading. 