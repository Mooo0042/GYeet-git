# GYeet - VotV Patcher GUI

A modern, smooth GTK4-based GUI for the YeetPatch VotV patcher with Proton support for Linux.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

## Features

- 🎨 **Modern GTK4 Interface** - Sleek, native Linux UI with dark theme
- 🔄 **Native Patching** - Pure Rust implementation, no bash scripts needed!
- 📦 **Version Selection** - Choose which VotV version to install
- 🎮 **Proton Integration** - Launch VotV using Steam's Proton compatibility layer
- ⚙️ **Configuration Management** - Automatic settings persistence
- 📊 **Real-time Console Output** - See what's happening during operations
- 🧵 **Thread-safe Operations** - Non-blocking UI during long operations
- 🔐 **SHA256 Verification** - Automatic integrity checking of downloads

## Prerequisites

Before building GYeet, you need:

1. **Rust** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **GTK 4** development libraries
   ```bash
   # Debian/Ubuntu
   sudo apt install libgtk-4-dev build-essential

   # Arch Linux
   sudo pacman -S gtk4

   # Fedora
   sudo dnf install gtk4-devel
   ```

3. **p7zip-full** - For extracting patch archives
   ```bash
   # Debian/Ubuntu
   sudo apt install p7zip-full

   # Arch Linux
   sudo pacman -S p7zip

   # Fedora
   sudo dnf install p7zip p7zip-plugins
   ```

4. **Steam with Proton** (for launching games)

## Building

```bash
# Clone or navigate to the repository
cd GYeet

# Build the project
cargo build --release

# The binary will be at target/release/gyeet
```

## Running

```bash
# Run directly with cargo
cargo run --release

# Or run the compiled binary
./target/release/gyeet
```

## Usage

### First Time Setup

1. **Settings Tab**:
   - Set the path to `YeetPatch.sh` (default: `~/GYeet/YeetPatch-latest-linux/YeetPatch.sh`)
   - Set your Steam installation path (default: `~/.steam/steam`)
   - Click "Save Settings"

### Patching an Existing Installation

1. Go to the **Patch/Update** tab
2. Click "Browse..." and select your `VotV.exe` file
3. Click "Check for Updates & Patch"
4. Watch the console output for progress

### Installing VotV

1. Go to the **Install** tab
2. Click "Browse..." and select where you want to install VotV
3. Click "Install VotV"
4. Follow the prompts in the console

### Launching with Proton

1. Go to the **Launch Game** tab
2. Click "Detect Proton Installations" to find available Proton versions
3. Select your preferred Proton version (or use Auto-detect)
4. Click "Launch VotV with Proton"

## Configuration

Settings are automatically saved to:
- Linux: `~/.config/gyeet/config.json`

The configuration file stores:
- VotV.exe path
- Installation directory
- YeetPatch script path
- Steam path
- Preferred Proton version

## Project Structure

```
GYeet/
├── src/
│   ├── main.rs          # Application entry point
│   ├── ui.rs            # Qt GUI implementation
│   ├── config.rs        # Configuration management
│   ├── patcher.rs       # YeetPatch wrapper
│   └── proton.rs        # Proton integration
├── YeetPatch-latest-linux/  # Original YeetPatch
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Dependencies

- `gtk4` - GTK 4 bindings for Rust
- `serde` - Serialization/deserialization
- `serde_json` - JSON support
- `dirs` - Platform-specific directories

## Troubleshooting

### GTK libraries not found
Make sure GTK 4 development libraries are installed:
```bash
# Debian/Ubuntu
sudo apt install libgtk-4-dev build-essential
```

### YeetPatch script not working
Ensure the YeetPatch script has execute permissions:
```bash
chmod +x YeetPatch-latest-linux/YeetPatch.sh
```

### Proton not detected
Make sure Steam is installed and the path in Settings points to your Steam directory (usually `~/.steam/steam` or `~/.local/share/Steam`).

## Credits

- **YeetPatch** - Original VotV patcher by the VotV community
- **VotV** - Voices of the Void game
- **Proton** - Valve's Windows compatibility layer

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

