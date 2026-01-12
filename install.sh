#!/bin/bash

# GYeet Installer Script
# This script installs GYeet from the latest release on Codeberg

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://codeberg.org/Fr4gm3nt3d_sh/GYeet"
INSTALL_DIR="$HOME/.local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"

echo -e "${GREEN}GYeet Installer${NC}"
echo "================================"

# Check dependencies
echo -e "\n${YELLOW}Checking dependencies...${NC}"
for cmd in curl jq; do
    if ! command -v $cmd &> /dev/null; then
        echo -e "${RED}Error: $cmd is not installed.${NC}"
        echo "Please install $cmd and try again."
        exit 1
    fi
done

# Create directories
echo -e "\n${YELLOW}Creating installation directories...${NC}"
mkdir -p "$INSTALL_DIR"
mkdir -p "$DESKTOP_DIR"
mkdir -p "$ICON_DIR"

# Fetch latest release info
echo -e "\n${YELLOW}Fetching latest release information...${NC}"
LATEST_RELEASE=$(curl -s "https://codeberg.org/api/v1/repos/Fr4gm3nt3d_sh/GYeet/releases/latest")

if [ -z "$LATEST_RELEASE" ]; then
    echo -e "${RED}Error: Could not fetch release information${NC}"
    exit 1
fi

VERSION=$(echo "$LATEST_RELEASE" | jq -r '.tag_name')
echo -e "${GREEN}Latest version: $VERSION${NC}"

# Get download URL for gyeet-linux
DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | jq -r '.assets[] | select(.name == "gyeet-linux") | .browser_download_url' | head -n1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Error: Could not find download URL${NC}"
    echo "Available assets:"
    echo "$LATEST_RELEASE" | jq -r '.assets[].name'
    exit 1
fi

echo -e "Download URL: $DOWNLOAD_URL"

# Download the binary
echo -e "\n${YELLOW}Downloading GYeet...${NC}"
TMP_FILE=$(mktemp)
if ! curl -L -o "$TMP_FILE" "$DOWNLOAD_URL"; then
    echo -e "${RED}Error: Download failed${NC}"
    rm -f "$TMP_FILE"
    exit 1
fi

# Install the binary
echo -e "\n${YELLOW}Installing binary...${NC}"
chmod +x "$TMP_FILE"
mv "$TMP_FILE" "$INSTALL_DIR/gyeet"
echo -e "${GREEN}Binary installed to $INSTALL_DIR/gyeet${NC}"

# Download and install icon
echo -e "\n${YELLOW}Downloading icon...${NC}"
ICON_URL="https://codeberg.org/Fr4gm3nt3d_sh/GYeet/raw/branch/main/assets/votv.png"
if curl -L -o "$ICON_DIR/gyeet.png" "$ICON_URL"; then
    echo -e "${GREEN}Icon installed to $ICON_DIR/gyeet.png${NC}"
else
    echo -e "${YELLOW}Warning: Could not download icon${NC}"
fi

# Create .desktop file
echo -e "\n${YELLOW}Creating desktop entry...${NC}"
cat > "$DESKTOP_DIR/gyeet.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=GYeet
Comment=GYeet Application
Exec=$INSTALL_DIR/gyeet
Icon=gyeet
Terminal=false
Categories=Utility;
EOF

chmod +x "$DESKTOP_DIR/gyeet.desktop"
echo -e "${GREEN}Desktop entry created${NC}"

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
fi

# Check if installation directory is in PATH
echo -e "\n${YELLOW}Checking PATH...${NC}"
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Warning: $INSTALL_DIR is not in your PATH${NC}"
    echo "Add the following line to your ~/.bashrc or ~/.zshrc:"
    echo -e "${GREEN}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    echo ""
    echo "Then run: source ~/.bashrc (or ~/.zshrc)"
fi

echo -e "\n${GREEN}================================${NC}"
echo -e "${GREEN}Installation complete!${NC}"
echo -e "${GREEN}================================${NC}"
echo ""
echo "You can now run GYeet with: gyeet"
echo "Or launch it from your application menu"