# Wake-on-LAN Manager (`wol-gui`)

A lightweight, cross-platform Wake-on-LAN (WoL) desktop application written in Rust using [egui](https://github.com/emilk/egui). It is designed for quick device wake-ups, local profile management, and a smooth native experience on Linux and Windows.

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)
[![Latest release](https://img.shields.io/github/v/release/dietmarschnabel-code/wol-gui?display_name=tag)](https://github.com/dietmarschnabel-code/wol-gui/releases/latest)

---

## Overview

`wol-gui` helps you manage network devices that can be started remotely via Wake-on-LAN magic packets. The app stores device profiles locally, lets you configure broadcast and UDP details, and makes it easy to send wake requests to hosts on the same local network.

---

## Features

- **Profile management:** Add, edit, select, and remove target device configurations.
- **Automatic persistence:** Profiles are saved locally across sessions using `eframe` persistence (`~/.config/wol-gui/app.ron` on Linux, `%APPDATA%\wol-gui\app.ron` on Windows).
- **Custom networking:** Configure broadcast addresses such as `192.168.1.255` or use subnet-wide broadcast `255.255.255.255`, and specify a custom UDP port (default: `9`).
- **Cross-platform support:** Built to run on Linux and Windows from a single codebase.
- **Internationalization (i18n):** The UI supports multiple languages for a more accessible user experience.
- **Theme support:** Switch between different visual themes, including light and dark modes, to match your desktop preference.

---

## Installation

For installation instructions, package builds, uninstallation steps, and release-specific guidance, see [INSTALL.md](INSTALL.md).

### Running from source

Ensure you have Rust installed ([rustup.rs](https://rustup.rs/)):

```bash
# Clone the repository
git clone https://github.com/dietmarschnabel-code/wol-gui.git
cd wol-gui

# Run the GUI application
cargo run --release
```

---

## Usage

1. Launch the application.
2. Add or select a target device profile.
3. Enter the target host details and optional broadcast settings.
4. Send the magic packet to wake the machine remotely.

---

## Project status

This application is actively maintained and continues to evolve with improvements such as localization and theme support.
