#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use fluent_templates::{static_loader, Loader};
use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

// Embed locales folder at compile time
static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

/// Detects the host operating system's language setting and maps it to a supported locale.
fn detect_system_language() -> String {
    let sys_lang = sys_locale::get_locale().unwrap_or_default();
    let normalized = sys_lang.replace('_', "-");

    // Direct match if OS reports exact locale code (e.g., "de-DE", "zh-CN")
    if matches!(
        normalized.as_str(),
        "en-US" | "es-ES" | "fr-FR" | "de-DE" | "zh-CN"
    ) {
        return normalized;
    }

    // Match base language prefix (e.g., "de_AT" -> "de" -> "de-DE")
    let lang_prefix = normalized.split('-').next().unwrap_or("").to_lowercase();
    match lang_prefix.as_str() {
        "es" => "es-ES".to_string(),
        "fr" => "fr-FR".to_string(),
        "de" => "de-DE".to_string(),
        "zh" => "zh-CN".to_string(),
        "en" => "en-US".to_string(),
        _ => "en-US".to_string(), // Default fallback
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ThemeMode {
    System,
    Dark,
    Light,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetDevice {
    pub name: String,
    pub mac: String,
    pub ip: String,
    pub port: u16,
}

impl Default for TargetDevice {
    fn default() -> Self {
        Self {
            name: "Workstation".to_string(),
            mac: "00:11:22:33:44:55".to_string(),
            ip: "255.255.255.255".to_string(),
            port: 9,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct WolApp {
    devices: Vec<TargetDevice>,
    selected_index: Option<usize>,
    current_lang: String,
    theme_mode: ThemeMode,

    #[serde(skip)]
    status_message: String,
    #[serde(skip)]
    is_error: bool,
}

impl Default for WolApp {
    fn default() -> Self {
        Self {
            devices: vec![TargetDevice::default()],
            selected_index: Some(0),
            current_lang: detect_system_language(), // Auto-detected on first launch
            theme_mode: ThemeMode::System,
            status_message: String::new(),
            is_error: false,
        }
    }
}

impl WolApp {
    fn tr(&self, key: &str) -> String {
        let lang: LanguageIdentifier = self
            .current_lang
            .parse()
            .unwrap_or_else(|_| "en-US".parse().unwrap());

        LOCALES
            .lookup(&lang, key)
            .unwrap_or_else(|| key.to_string())
    }

    fn tr_args(&self, key: &str, args: &HashMap<&str, &str>) -> String {
        let lang: LanguageIdentifier = self
            .current_lang
            .parse()
            .unwrap_or_else(|_| "en-US".parse().unwrap());
        let fluent_args: HashMap<String, fluent_templates::fluent_bundle::FluentValue> = args
            .iter()
            .map(|(k, v)| (k.to_string(), (*v).into()))
            .collect();

        LOCALES
            .lookup_with_args(&lang, key, &fluent_args)
            .unwrap_or_else(|| key.to_string())
    }

    fn send_packet(&mut self, index: usize) {
        let device = match self.devices.get(index) {
            Some(d) => d.clone(),
            None => return,
        };

        match MacAddr6::from_str(&device.mac) {
            Ok(mac) => {
                let bytes = mac.into_array();
                let mut packet = [0xFFu8; 102];
                for i in 0..16 {
                    let start = 6 + i * 6;
                    packet[start..start + 6].copy_from_slice(&bytes);
                }

                match UdpSocket::bind("0.0.0.0:0") {
                    Ok(socket) => {
                        let _ = socket.set_broadcast(true);
                        let dest_str = format!("{}:{}", device.ip, device.port);

                        match dest_str.parse::<SocketAddr>() {
                            Ok(destination) => {
                                if let Err(e) = socket.send_to(&packet, destination) {
                                    self.status_message = format!("Socket error: {}", e);
                                    self.is_error = true;
                                } else {
                                    let mut args = HashMap::new();
                                    args.insert("name", device.name.as_str());
                                    args.insert("mac", device.mac.as_str());
                                    self.status_message = self.tr_args("status-success", &args);
                                    self.is_error = false;
                                }
                            }
                            Err(_) => {
                                self.status_message = self.tr("status-invalid-ip");
                                self.is_error = true;
                            }
                        }
                    }
                    Err(e) => {
                        self.status_message = format!("Socket error: {}", e);
                        self.is_error = true;
                    }
                }
            }
            Err(_) => {
                self.status_message = self.tr("status-invalid-mac");
                self.is_error = true;
            }
        }
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.theme_mode {
            ThemeMode::Dark => ctx.set_visuals(egui::Visuals::dark()),
            ThemeMode::Light => ctx.set_visuals(egui::Visuals::light()),
            ThemeMode::System => {}
        }
    }
}

impl eframe::App for WolApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        // Pre-evaluate strings into local variables to satisfy Rust borrow checker
        let app_title = self.tr("app-title");
        let theme_label = self.tr("theme-label");
        let theme_dark = self.tr("theme-dark");
        let theme_light = self.tr("theme-light");
        let sidebar_devices = self.tr("sidebar-devices");
        let sidebar_add = self.tr("sidebar-add-device");
        let label_name = self.tr("label-name");
        let label_mac = self.tr("label-mac");
        let label_ip = self.tr("label-ip");
        let label_port = self.tr("label-port");
        let btn_send = self.tr("btn-send");

        // Top Header Bar: Language & Theme Controls
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(app_title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Theme Selector
                    ui.selectable_value(&mut self.theme_mode, ThemeMode::Light, theme_light);
                    ui.selectable_value(&mut self.theme_mode, ThemeMode::Dark, theme_dark);
                    ui.label(theme_label);

                    ui.separator();

                    // Language Selector
                    egui::ComboBox::from_id_source("lang_selector")
                        .selected_text(match self.current_lang.as_str() {
                            "es-ES" => "Español",
                            "fr-FR" => "Français",
                            "de-DE" => "Deutsch",
                            "zh-CN" => "中文",
                            _ => "English",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.current_lang, "en-US".to_string(), "English");
                            ui.selectable_value(&mut self.current_lang, "es-ES".to_string(), "Español");
                            ui.selectable_value(&mut self.current_lang, "fr-FR".to_string(), "Français");
                            ui.selectable_value(&mut self.current_lang, "de-DE".to_string(), "Deutsch");
                            ui.selectable_value(&mut self.current_lang, "zh-CN".to_string(), "中文");
                        });
                });
            });
        });

        // Left Sidebar: Device List
        egui::SidePanel::left("devices_panel")
            .resizable(false)
            .default_width(180.0)
            .show(ctx, |ui| {
                ui.heading(sidebar_devices);
                ui.separator();

                let mut to_remove = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, dev) in self.devices.iter().enumerate() {
                        ui.horizontal(|ui| {
                            let is_selected = self.selected_index == Some(i);
                            if ui.selectable_label(is_selected, &dev.name).clicked() {
                                self.selected_index = Some(i);
                            }
                            if ui.button("❌").clicked() {
                                to_remove = Some(i);
                            }
                        });
                    }
                });

                ui.separator();

                if ui.button(sidebar_add).clicked() {
                    self.devices.push(TargetDevice::default());
                    self.selected_index = Some(self.devices.len() - 1);
                }

                if let Some(idx) = to_remove {
                    self.devices.remove(idx);
                    if self.devices.is_empty() {
                        self.selected_index = None;
                    } else if let Some(selected) = self.selected_index {
                        if selected >= self.devices.len() {
                            self.selected_index = Some(self.devices.len() - 1);
                        }
                    }
                }
            });

        // Central Panel: Configuration Form
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.selected_index {
                if let Some(device) = self.devices.get_mut(idx) {
                    egui::Grid::new("device_form")
                        .num_columns(2)
                        .spacing([20.0, 10.0])
                        .show(ui, |ui| {
                            ui.label(label_name);
                            ui.text_edit_singleline(&mut device.name);
                            ui.end_row();

                            ui.label(label_mac);
                            ui.text_edit_singleline(&mut device.mac);
                            ui.end_row();

                            ui.label(label_ip);
                            ui.text_edit_singleline(&mut device.ip);
                            ui.end_row();

                            ui.label(label_port);
                            ui.add(egui::DragValue::new(&mut device.port).clamp_range(1..=65535));
                            ui.end_row();
                        });

                    ui.add_space(20.0);

                    if ui
                        .add_sized([ui.available_width(), 38.0], egui::Button::new(btn_send))
                        .clicked()
                    {
                        self.send_packet(idx);
                    }
                }
            }

            ui.add_space(15.0);
            ui.separator();

            let color = if self.is_error {
                egui::Color32::LIGHT_RED
            } else {
                egui::Color32::LIGHT_GREEN
            };
            ui.colored_label(color, &self.status_message);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([620.0, 360.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Wake-on-LAN Manager",
        options,
        Box::new(|cc| {
            if let Some(storage) = cc.storage {
                if let Some(app) = eframe::get_value::<WolApp>(storage, eframe::APP_KEY) {
                    return Box::new(app);
                }
            }
            Box::<WolApp>::default()
        }),
    )
}