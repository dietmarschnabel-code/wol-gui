# Installation, Uninstallation, and Packaging Guide for `wol-gui`

This guide explains how to install the application from official releases, remove it cleanly from your system, and build native packages or a standalone binary from source.

---

## Table of Contents

1. [Installing pre-built releases](#1-installing-pre-built-releases)
   - [Fedora / RHEL / CentOS (.rpm)](#fedora--rhel--centos-rpm)
   - [Debian / Ubuntu / Linux Mint (.deb)](#debian--ubuntu--linux-mint-deb)
   - [Windows (.msi / .zip)](#windows-msi--zip)
2. [Uninstallation instructions](#2-uninstallation-instructions)
   - [Fedora / RHEL](#fedora--rhel)
   - [Debian / Ubuntu](#debian--ubuntu)
   - [Windows](#windows)
3. [Building packages from source](#3-building-packages-from-source)
   - [Prerequisites](#prerequisites)
   - [Fedora RPM package](#fedora-rpm-package)
   - [Debian / Ubuntu DEB package](#debian--ubuntu-deb-package)
   - [Windows MSI installer](#windows-msi-installer)
4. [Building a standalone binary from source](#4-building-a-standalone-binary-from-source)

---

## 1. Installing pre-built releases

Official release artifacts for supported platforms are published on the [GitHub Releases](https://github.com/dietmarschnabel-code/wol-gui/releases) page.
Artifact version numbers match the release tag without its leading `v` (for example, tag `vX.Y.Z` produces assets using `X.Y.Z`).

### Fedora / RHEL / CentOS (.rpm)

Download the latest `.rpm` package, then install it with `dnf`:

```bash
sudo dnf install ./wol-gui-<release-version>-1.<fedora-release>.<architecture>.rpm
```

### Debian / Ubuntu / Linux Mint (.deb)

Download the latest `.deb` package, then install it with `apt`:

```bash
sudo apt update
sudo apt install ./wol-gui_<release-version>_amd64.deb
```

If necessary, you can install it using `dpkg` and then fix any missing dependencies:

```bash
sudo dpkg -i wol-gui_<release-version>_amd64.deb
sudo apt-get install -f
```

### Windows (.msi / .zip)

- MSI installer (recommended): download `wol-gui-<release-version>-x86_64.msi` and run the installer. This creates a Start Menu shortcut and adds an uninstaller entry in Windows Settings.
- Portable ZIP archive: download `wol-gui-windows-x86_64.zip`, extract it, and run `wol-gui.exe` directly.

---

## 2. Uninstallation instructions

### Fedora / RHEL

Remove the package and its system-level application shortcuts:

```bash
sudo dnf remove wol-gui
```

### Debian / Ubuntu

Remove the package while keeping user configuration files intact:

```bash
sudo apt remove wol-gui
```

Remove the package and its configuration files completely:

```bash
sudo apt purge wol-gui
```

### Windows

Via Settings:

1. Open Settings.
2. Go to Apps → Installed apps (or Apps & features).
3. Find Wake-on-LAN Manager.
4. Open the options menu (`...`) and select Uninstall.

Via Control Panel:

1. Open Control Panel.
2. Go to Programs and Features.
3. Right-click Wake-on-LAN Manager and select Uninstall.

---

## 3. Building packages from source

### Prerequisites

To build packages, install the Rust toolchain using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Fedora, install the RPM build tooling and basic dependencies:

```bash
sudo dnf install -y rpmdevtools cargo desktop-file-utils git
```

### Fedora RPM package

Set up the local RPM build directory structure:

```bash
rpmdev-setuptree
```

Create a source tarball that matches the spec file version:

```bash
VERSION=$(git describe --tags --exact-match --match 'v*' | sed 's/^v//')
git archive --format=tar.gz --prefix="wol-gui-${VERSION}/" "v${VERSION}" -o ~/rpmbuild/SOURCES/wol-gui-${VERSION}.tar.gz
sed -i "s/^Version:.*/Version:        ${VERSION}/" wol-gui.spec
```

Copy the spec file into the RPM build tree and build the package:

```bash
cp wol-gui.spec ~/rpmbuild/SPECS/
rpmbuild -ba ~/rpmbuild/SPECS/wol-gui.spec
```

The built package will be created at:

```text
~/rpmbuild/RPMS/<architecture>/wol-gui-<release-version>-1.<fedora-release>.<architecture>.rpm
```

### Debian / Ubuntu DEB package

#### Method A: Automated build with `cargo-deb` (recommended)

Install `cargo-deb`:

```bash
cargo install cargo-deb
```

Ensure your `Cargo.toml` includes a `[package.metadata.deb]` section similar to the following:

```toml
[package.metadata.deb]
maintainer = "Your Name <your.email@example.com>"
copyright = "2026, Your Name <your.email@example.com>"
license-file = ["LICENSE-MIT", "4"]
extended-description = """\
A lightweight, memory-safe Wake-on-LAN desktop manager built in Rust.
Allows managing target device profiles and sending magic packets across subnets.
"""
section = "net"
priority = "optional"
assets = [
    ["target/release/wol-gui", "usr/bin/", "755"],
    ["wol-gui.desktop", "usr/share/applications/", "644"],
]
```

Build the package:

```bash
VERSION=$(git describe --tags --exact-match --match 'v*' | sed 's/^v//')
sed -i "0,/^version = .*/s//version = \"${VERSION}\"/" Cargo.toml
cargo deb
```

The output package will be created in:

```text
target/debian/wol-gui_${VERSION}_amd64.deb
```

#### Method B: Manual build with `dpkg-deb`

Compile the application in release mode:

```bash
VERSION=$(git describe --tags --exact-match --match 'v*' | sed 's/^v//')
sed -i "0,/^version = .*/s//version = \"${VERSION}\"/" Cargo.toml
cargo build --release
```

Create the package directory structure:

```bash
mkdir -p "wol-gui_${VERSION}_amd64/DEBIAN"
mkdir -p "wol-gui_${VERSION}_amd64/usr/bin"
mkdir -p "wol-gui_${VERSION}_amd64/usr/share/applications"

cp target/release/wol-gui "wol-gui_${VERSION}_amd64/usr/bin/"
cp wol-gui.desktop "wol-gui_${VERSION}_amd64/usr/share/applications/"
```

Create the Debian control file:

```bash
cat <<EOF > "wol-gui_${VERSION}_amd64/DEBIAN/control"
Package: wol-gui
Version: ${VERSION}
Section: net
Priority: optional
Architecture: amd64
Maintainer: Your Name <your.email@example.com>
Description: Cross-platform Wake-on-LAN GUI manager written in Rust
 A lightweight desktop application to send magic packets to remote machines.
EOF
```

Build the package:

```bash
dpkg-deb --build "wol-gui_${VERSION}_amd64"
```

The resulting package is saved as:

```text
./wol-gui_${VERSION}_amd64.deb
```

### Windows MSI installer

Install WiX Toolset v3.11 or newer:

```powershell
winget install WiXToolset.WiXToolset
```

Install the `cargo-wix` helper tool:

```powershell
cargo install cargo-wix
```

Build the MSI installer:

```powershell
$version = (git describe --tags --exact-match --match "v*").TrimStart("v")
$content = Get-Content Cargo.toml -Raw
$content = $content -replace '(?m)^version = "[^"]+"$', "version = `"$version`""
Set-Content Cargo.toml -Value $content -NoNewline
cargo wix --release
```

The installer is generated at:

```text
target/wix/wol-gui-<release-version>-x86_64.msi
```

---

## 4. Building a standalone binary from source

To build a direct executable without creating an OS package:

```bash
# Clone the repository
git clone https://github.com/dietmarschnabel-code/wol-gui.git
cd wol-gui
git fetch --tags
git checkout "$(git tag --list 'v*' --sort=-version:refname | head -n 1)"

# Set the Cargo package version from the checked-out release tag
VERSION=$(git describe --tags --exact-match --match 'v*' | sed 's/^v//')
sed -i "0,/^version = .*/s//version = \"${VERSION}\"/" Cargo.toml

# Compile in release mode
cargo build --release
```

The standalone binary will be generated at:

- Linux: `target/release/wol-gui`
- Windows: `target/release/wol-gui.exe`

This output is intended for direct execution and does not install application metadata or desktop integration entries on the system.
