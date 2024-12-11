# Awake

A cross-platform system tray application that prevents your system from going to sleep. Built with Rust and Tauri.

![Awake Icon](src-tauri/icons/icon-allow-32x32.png)

## Features

- Prevent system sleep with a single click
- System tray integration for easy access
- Start at login option
- Cross-platform support (Windows, macOS, Linux)
- Minimal resource usage
- No visible interference with your work

## Installation

### Pre-built Binaries
Download the latest release for your platform from the [Releases](https://github.com/yourusername/awake/releases) page.

### Building from Source

#### Prerequisites
- [Rust](https://rustup.rs/) (1.70.0 or later)
- [Node.js](https://nodejs.org/) (18.0.0 or later)
- Platform-specific dependencies for Tauri - [See Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

#### Build Steps
1. Clone the repository
   ```bash
   git clone https://github.com/yourusername/awake.git
   cd awake
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
4. Optionally enable "Start at Login" for automatic startup

## How it Works

Awake uses a non-intrusive method to keep your system awake by simulating a function key (F15) press every 60 seconds. This method:
- Doesn't interfere with your work
- Doesn't prevent screen dimming (only prevents sleep)
- Works consistently across all supported platforms

## Development

### Project Structure
```
awake/
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

- Built with [Tauri](https://tauri.app/)
- Uses [enigo](https://github.com/enigo-rs/enigo) for cross-platform input simulation
