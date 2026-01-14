# GYeet - VotV Patcher GUI

<p align="center">
  <img src="assets/votv.png" alt="GYeet Logo" width="200"/>
</p>

A modern, smooth GTK4-based GUI for the YeetPatch VotV patcher with Proton support for Linux.

![License](https://img.shields.io/badge/license-GPLv3-purple.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

## Direct Installation (Linux)

To quickly install GYeet on Linux, execute the following command in your terminal:

```bash
curl -fsSL https://codeberg.org/Fr4gm3nt3d_sh/GYeet/raw/branch/main/install.sh | bash
```
This script will download the latest pre-compiled binary and install it to `/usr/local/bin`.

## Features

- ✅ Modern GTK4 interface
- ✅ Dark theme
- ✅ **Native installation** - Downloads and installs VotV using desync
- ✅ **Version selection** - Choose any available VotV version
- ✅ Real-time console output
- ✅ Automatic settings persistence
- ✅ SHA256 verification for downloads
- ✅ Proton version detection
- ✅ Thread-safe operations

## Building and Installation from Source

To build and install GYeet from source, follow these steps.

### Prerequisites (Dependencies)

GYeet requires the following development libraries and tools:

-   **Rust and Cargo**: Install via `rustup` (recommended): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
-   **GTK 4 Development Libraries**:

    -   **Debian/Ubuntu/Pop!_OS**:
        ```bash
        sudo apt update
        sudo apt install libgtk-4-dev build-essential pkg-config
        ```
    -   **Fedora**:
        ```bash
        sudo dnf install gtk4-devel gcc
        ```
    -   **Arch Linux**:
        ```bash
        sudo pacman -S gtk4 base-devel
        ```
    -   **Other Linux Distributions**: Please refer to your distribution's documentation for installing GTK 4 development packages.

### Build and Install

Once the prerequisites are installed, you can build and install GYeet:

1.  **Clone the repository**:
    ```bash
    git clone https://codeberg.org/Fr4gm3nt3d_sh/GYeet.git
    cd GYeet
    ```

2.  **Build and install**:
    ```bash
    sudo make install
    ```
    This command will compile the project in release mode and install the `gyeet` executable to `/usr/local/bin`. You can change the installation directory by setting the `INSTALL_DIR` variable, for example: `make install INSTALL_DIR=~/bin`.

## Usage

### First Time Setup

1.  **Open Settings Tab**
    -   Set Steam path (usually `~/.steam/steam` or `~/.local/share/Steam`)
    -   Click "Save Settings"
2.  **Configure Game Path** (if you have an existing installation)
    -   Go to "Patch/Update" tab
    -   Click "Browse..." and select your `VotV.exe` file
    -   This path will be saved automatically

### Patching/Updating VotV

1.  Go to "Patch/Update" tab
2.  Select `VotV.exe` if not already set
3.  Click "Check for Updates & Patch"
4.  Watch console output for progress

### Installing VotV

1.  Go to "Install" tab
2.  Select installation directory
3.  Click "🔄 Refresh" to load available game versions
4.  Select your desired version from the dropdown
5.  Click "Install VotV"
6.  Watch console output for progress

### Launching with Proton

1.  Go to "Launch Game" tab
2.  Click "Detect Proton Installations" to find available Proton versions
3.  Select your preferred Proton version
4.  Click "Launch VotV with Proton"

## Troubleshooting

### General Build Errors

-   Ensure all prerequisites listed above are correctly installed.
-   Verify your GTK4 development libraries are installed and accessible by `pkg-config`. You can check the GTK4 version with: `pkg-config --modversion gtk4`.

### Patch Download Fails

-   Check your internet connection.
-   Ensure `p7zip-full` is installed (needed for unpacking archives by the underlying `YeetPatch` logic).
-   Check the console output within the GYeet application for specific error messages.

### Proton Not Detected

-   Verify Steam is installed on your system.
-   Check the Steam path configured in GYeet's Settings tab. Common paths include `~/.steam/steam` or `~/.local/share/Steam`.

### Game Won't Launch

-   Ensure the `VotV.exe` path configured in GYeet is correct.
-   Check that Proton is properly installed and managed by Steam.
-   Try launching with different Proton versions available.

## Credits

-   **YeetPatch** - Original VotV patcher
-   **VotV** - Voices of the Void game
-   **Proton** - Valve's Windows compatibility layer

## License

This project is licensed under the GPLv3 License. See the `LICENSE` file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests on the [Codeberg repository](https://codeberg.org/Fr4gm3nt3d_sh/GYeet).