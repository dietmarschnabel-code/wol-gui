# Wake-on-LAN Manager (`wol-gui`)

A lightweight, cross-platform Wake-on-LAN (WoL) desktop application written in Rust using [egui](https://github.com/emilk/egui). Designed for ease of use, memory efficiency, and native integration on both Linux (Fedora/Wayland/X11) and Windows.

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)

---

## Features

- **Profile Management:** Add, edit, select, and delete target device configurations.
- **Automatic Persistence:** Target profiles are automatically saved locally across sessions using `eframe` persistence (`~/.config/wol-gui/app.ron` on Linux, `%APPDATA%\wol-gui\app.ron` on Windows).
- **Custom Broadcast Settings:** Configure specific broadcast IPs (e.g., `192.168.1.255`) or use global subnet broadcast (`255.255.255.255`), along with custom target UDP ports (default: 9).
- **Cross-Platform:** Single binary release for both Windows and Linux environments.

---

## Installation & Usage

### Running from Source

Ensure you have Rust installed ([rustup.rs](https://rustup.rs/)):

```bash
# Clone the repository
git clone [https://github.com/yourusername/wol-gui.git](https://github.com/yourusername/wol-gui.git)
cd wol-gui

# Run the GUI application
cargo run --release