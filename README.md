# Tea

A cross-platform system tray application that prevents your system from going to sleep. Built with Rust and Tauri 2.0.

![Tea Icon](src-tauri/icons/icon-allow-32x32.png)

## Features

- Prevent system sleep with a single click
- **Screen Control Modes:**
  - **Keep Screen On**: Prevents both system sleep and screen turning off (Windows only)
  - **Allow Screen Off**: Keeps system awake but allows screen to sleep/turn off
- System tray integration for easy access
- Start at login option
- Cross-platform support (Windows, macOS, Linux)
- Minimal resource usage
- No visible interference with your work

## Installation

### Pre-built Binaries
Download the latest release for your platform from the [Releases](https://github.com/Swatto86/tea/releases) page.

### Building from Source

#### Prerequisites
- [Rust](https://rustup.rs/) (1.70.0 or later)
- [Node.js](https://nodejs.org/) (18.0.0 or later)
- Platform-specific dependencies for Tauri - [See Tauri Prerequisites](https://tauri.app/v2/guides/getting-started/prerequisites)

#### Build Steps
1. Clone the repository
   ```bash
   git clone https://github.com/Swatto86/tea.git
   cd tea
   ```

2. Build the application
   ```bash
   cargo tauri build
   ```

The compiled application will be available in `src-tauri/target/release`.

## Usage

1. Launch the application
2. Click the system tray icon (appears in your taskbar/menu bar)
3. Select "Disable Sleep" to prevent your system from sleeping
4. Choose your screen mode:
   - **Keep Screen On**: Prevents screen from turning off (Windows: uses native API)
   - **Allow Screen Off**: Lets screen sleep but keeps system awake
5. Optionally enable "Start at Login" for automatic startup

## How it Works

Tea uses an intelligent approach combining F15 key simulation with platform-specific display control:

### Windows Platform
- **Keep Screen On** mode: Uses Windows `SetThreadExecutionState` API with `ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED` flags + F15 simulation for redundancy
- **Allow Screen Off** mode: Uses only the Windows API with `ES_SYSTEM_REQUIRED` flag (no F15), which keeps the system awake while allowing the screen to sleep normally

### Non-Windows Platforms (macOS, Linux)
- Simulates a function key (F15) press every 60 seconds to prevent system sleep
- Non-intrusive method that doesn't interfere with your work
- **Only "Keep Screen On" mode available** - F15 simulation prevents both system and display sleep, making "Allow Screen Off" technically impossible on these platforms

### Benefits
- Minimal system impact with F15 key simulation
- Works reliably in the background on all platforms
- Additional Windows-specific screen control when needed
- Preserves your settings between sessions

## Development

### Project Structure
```
tea/
├── src-tauri/          # Rust backend code
│   ├── src/            # Source files
│   ├── icons/          # Application icons
│   └── Cargo.toml      # Rust dependencies
└── README.md           # This file
```

### Contributing
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri 2.0](https://tauri.app/)
- Uses [enigo](https://github.com/enigo-rs/enigo) for cross-platform input simulation
