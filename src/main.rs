use eframe::egui;
use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;

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
            name: "New Workstation".to_string(),
            mac: "00:11:22:33:44:55".to_string(),
            ip: "255.255.255.255".to_string(),
            port: 9,
        }
    }
}

/// Persistent application state saved automatically by eframe
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct WolApp {
    devices: Vec<TargetDevice>,
    selected_index: Option<usize>,

    // Transient UI state (not serialized)
    #[serde(skip)]
    status_message: String,
    #[serde(skip)]
    is_error: bool,
}

impl Default for WolApp {
    fn default() -> Self {
        Self {
            devices: vec![
                TargetDevice {
                    name: "Home PC".to_string(),
                    mac: "00:11:22:33:44:55".to_string(),
                    ip: "192.168.1.255".to_string(),
                    port: 9,
                },
                TargetDevice {
                    name: "NAS Server".to_string(),
                    mac: "AA:BB:CC:DD:EE:FF".to_string(),
                    ip: "255.255.255.255".to_string(),
                    port: 9,
                },
            ],
            selected_index: Some(0),
            status_message: "Select a device profile to manage.".to_string(),
            is_error: false,
        }
    }
}

impl WolApp {
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
                                    self.status_message = format!("Failed to send packet: {}", e);
                                    self.is_error = true;
                                } else {
                                    self.status_message = format!(
                                        "⚡ Magic packet sent to '{}' ({})!",
                                        device.name, device.mac
                                    );
                                    self.is_error = false;
                                }
                            }
                            Err(_) => {
                                self.status_message = "Invalid IP address or port.".to_string();
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
                self.status_message = "Invalid MAC format (expected 00:11:22:33:44:55).".to_string();
                self.is_error = true;
            }
        }
    }
}

impl eframe::App for WolApp {
    /// Save persistent state to disk when closing or when requested by eframe
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Left Sidebar: Saved Profiles List ---
        egui::SidePanel::left("devices_panel")
            .resizable(false)
            .default_width(170.0)
            .show(ctx, |ui| {
                ui.heading("Devices");
                ui.separator();

                let mut to_remove = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, dev) in self.devices.iter().enumerate() {
                        ui.horizontal(|ui| {
                            let is_selected = self.selected_index == Some(i);
                            if ui.selectable_label(is_selected, &dev.name).clicked() {
                                self.selected_index = Some(i);
                            }

                            // Delete button next to each profile
                            if ui.button("❌").clicked() {
                                to_remove = Some(i);
                            }
                        });
                    }
                });

                ui.separator();

                // Add new profile button
                if ui.button("➕ Add Device").clicked() {
                    self.devices.push(TargetDevice::default());
                    self.selected_index = Some(self.devices.len() - 1);
                }

                // Handle deletion outside iteration
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

        // --- Main Central Panel: Editor & Controls ---
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.selected_index {
                if let Some(device) = self.devices.get_mut(idx) {
                    ui.heading("Device Configuration");
                    ui.separator();

                    egui::Grid::new("device_form")
                        .num_columns(2)
                        .spacing([20.0, 10.0])
                        .show(ui, |ui| {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut device.name);
                            ui.end_row();

                            ui.label("MAC Address:");
                            ui.text_edit_singleline(&mut device.mac);
                            ui.end_row();

                            ui.label("Broadcast IP:");
                            ui.text_edit_singleline(&mut device.ip);
                            ui.end_row();

                            ui.label("Port:");
                            ui.add(egui::DragValue::new(&mut device.port).range(1..=65535));
                            ui.end_row();
                        });

                    ui.add_space(20.0);

                    if ui
                        .add_sized([ui.available_width(), 38.0], egui::Button::new("⚡ Send Magic Packet"))
                        .clicked()
                    {
                        self.send_packet(idx);
                    }
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("No device selected. Click '➕ Add Device' to get started.");
                });
            }

            ui.add_space(15.0);
            ui.separator();

            // Status bar output
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
            .with_inner_size([520.0, 320.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Wake-on-LAN Manager",
        options,
        Box::new(|cc| {
            // Restore persistent state from storage if available
            if let Some(storage) = cc.storage {
                if let Some(app) = eframe::get_value::<WolApp>(storage, eframe::APP_KEY) {
                    return Box::new(app);
                }
            }
            Box::<WolApp>::default()
        }),
    )
}