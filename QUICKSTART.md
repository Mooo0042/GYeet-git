# GYeet Quick Start Guide

## Installation

### 1. Install Dependencies

**Debian/Ubuntu:**
```bash
sudo apt install libgtk-4-dev build-essential curl p7zip-full tar
```

**Arch Linux:**
```bash
sudo pacman -S gtk4 base-devel curl p7zip tar
```

**Fedora:**
```bash
sudo dnf install gtk4-devel gcc curl p7zip p7zip-plugins tar
```

### 2. Install Rust (if not already installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Build GYeet
```bash
cd GYeet
./build.sh
```

Or manually:
```bash
cargo build --release
```

## Running GYeet

```bash
./run.sh
```

Or directly:
```bash
./target/release/gyeet
```

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

## Support

For issues or questions:
- Check the main README.md
- Review console output for error messages
- Ensure all dependencies are installed

