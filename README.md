# GYeet - VotV Patcher GUI

A modern, smooth GTK4-based GUI for the YeetPatch VotV patcher with Proton support for Linux.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

## Installation

### Linux
Execute the following command in your terminal:

```
curl -fsSL https://codeberg.org/Fr4gm3nt3d_sh/GYeet/raw/branch/main/install.sh | bash
```

### Windows
Currently not supported.

## First Time Setup

1. **Open Settings Tab**
   - Set Steam path (usually `~/.steam/steam` or `~/.local/share/Steam`)
   - Click "Save Settings"

2. **Configure Game Path** (if you have an existing installation)
   - Go to "Patch/Update" tab
   - Click "Browse..." and select your `VotV.exe` file
   - This path will be saved automatically

## Usage

### Patching/Updating VotV
1. Go to "Patch/Update" tab
2. Select VotV.exe if not already set
3. Click "Check for Updates & Patch"
4. Watch console output for progress

### Installing VotV
1. Go to "Install" tab
2. Select installation directory
3. Click "🔄 Refresh" to load available game versions
4. Select your desired version from the dropdown
5. Click "Install VotV"
6. Watch console output for progress

### Launching with Proton
1. Go to "Launch Game" tab
2. Click "Detect Proton Installations" to find available Proton versions
3. Select your preferred Proton version
4. Click "Launch VotV with Proton"

## Troubleshooting

### Build Errors
- Make sure GTK4 development libraries are installed
- Try: `pkg-config --modversion gtk4` to check GTK4 version

### Patch Download Fails
- Check your internet connection
- Ensure p7zip-full is installed
- Check console output for specific error messages

### Proton Not Detected
- Verify Steam is installed
- Check Steam path in Settings
- Common paths: `~/.steam/steam`, `~/.local/share/Steam`

### Game Won't Launch
- Ensure VotV.exe path is correct
- Check that Proton is properly installed in Steam
- Try different Proton versions

## Features

- ✅ Modern GTK4 interface
- ✅ Dark theme
- ✅ **Native Rust patching** - No bash scripts needed!
- ✅ **Native installation** - Downloads and installs VotV using desync
- ✅ **Version selection** - Choose any available VotV version
- ✅ Real-time console output
- ✅ Automatic settings persistence
- ✅ SHA256 verification for downloads
- ✅ Proton version detection
- ✅ Thread-safe operations


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

- **YeetPatch** - Original VotV patcher
- **VotV** - Voices of the Void game
- **Proton** - Valve's Windows compatibility layer

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

