# Makefile
# Variables
RUST_PROJECT_DIR := .
INSTALL_DIR ?= /usr/local
BUILD_DIR := $(RUST_PROJECT_DIR)/target/release
DESKTOP_DIR := $(HOME)/.local/share/applications
ICON_DIR := $(HOME)/.local/share/icons/hicolor/256x256/apps
APP_ICON := $(RUST_PROJECT_DIR)/assets/votv.png
APP_NAME := gyeet
APP_COMMENT := GYeet Application

# Default target: build the project
all: install

# Target to build the project
build:
	@echo "Building the project..."
	@cargo build --release

# Target to install the project
install: build
	@echo "Installing GYeet binary..."
	@mkdir -p $(INSTALL_DIR)/bin
	@cp $(BUILD_DIR)/$(APP_NAME) $(INSTALL_DIR)/bin/$(APP_NAME)
	@chmod +x $(INSTALL_DIR)/bin/$(APP_NAME)
	@echo "Installing desktop entry and icon..."
	@mkdir -p $(DESKTOP_DIR)
	@mkdir -p $(ICON_DIR)
	@cp $(APP_ICON) $(ICON_DIR)/$(APP_NAME).png
	@echo "[Desktop Entry]" > $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Version=1.0" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Type=Application" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Name=GYeet" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Comment=$(APP_COMMENT)" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Exec=$(INSTALL_DIR)/bin/$(APP_NAME)" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Icon=$(APP_NAME)" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Terminal=false" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@echo "Categories=Utility;" >> $(DESKTOP_DIR)/$(APP_NAME).desktop
	@chmod +x $(DESKTOP_DIR)/$(APP_NAME).desktop
	@if command -v update-desktop-database >/dev/null 2>&1; then update-desktop-database $(DESKTOP_DIR) >/dev/null 2>&1; fi

# Target to clean the build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

.PHONY: all build install clean