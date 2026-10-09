# Installation, Uninstallation & Packaging Guide (`wol-gui`)

This document provides complete instructions for installing, uninstalling, and building native packages (`.rpm`, `.deb`, `.msi`) from source across Fedora, Debian/Ubuntu, and Windows.

---

## Table of Contents

1. [Installing Pre-Built Releases](#1-installing-pre-built-releases)
   - [Fedora / RHEL / CentOS (.rpm)](#fedora--rhel--centos-rpm)
   - [Debian / Ubuntu / Linux Mint (.deb)](#debian--ubuntu--linux-mint-deb)
   - [Windows (.msi / .zip)](#windows-msi--zip)
2. [Uninstallation Instructions](#2-uninstallation-instructions)
   - [Fedora / RHEL](#fedora--rhel)
   - [Debian / Ubuntu](#debian--ubuntu)
   - [Windows](#windows)
3. [Building Packages from Source](#3-building-packages-from-source)
   - [Prerequisites](#prerequisites)
   - [Fedora RPM Package](#fedora-rpm-package)
   - [Debian/Ubuntu DEB Package](#debianubuntu-deb-package)
   - [Windows MSI Installer](#windows-msi-installer)
4. [Building Standalone Binary from Source](#4-building-standalone-binary-from-source)

---

## 1. Installing Pre-Built Releases

Pre-compiled binary packages for all supported operating systems are attached to every official release on the [GitHub Releases](../../releases) page.

### Fedora / RHEL / CentOS (.rpm)

Download the latest `.rpm` package and install it using `dnf`:

```bash
sudo dnf install ./wol-gui-0.1.0-1.fc40.x86_64.rpm