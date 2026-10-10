Name:           wol-gui
Version:        0.1.5
Release:        1%{?dist}
Summary:        Cross-platform Wake-on-LAN GUI manager written in Rust

License:        MIT OR Apache-2.0
URL:            https://github.com/yourusername/wol-gui
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo

%description
A lightweight, memory-safe Wake-on-LAN desktop application built with Rust and egui.
Allows managing multiple remote host profiles and sending UDP magic packets across networks.

%prep
%autosetup
sed -i '0,/^version = .*/s//version = "%{version}"/' Cargo.toml

%build
cargo build --release

%install
install -Dpm 0755 target/release/wol-gui %{buildroot}%{_bindir}/wol-gui
if [ -f %{name}.desktop ]; then
    install -Dpm 0644 %{name}.desktop %{buildroot}%{_datadir}/applications/%{name}.desktop
fi

%files
%{_bindir}/wol-gui
%{?_datadir}/applications/%{name}.desktop

%changelog
* Fri Oct 09 2026 Developer <dev@example.com> - %{version}-%{release}
- Tagged Automated Release
