use egui::Grid;
use evergreen::evergreen::Evergreen;
use pcap::Device;

use crate::session::Session;

type ArcMut<T> = std::sync::Arc<std::sync::Mutex<T>>;
macro_rules! arc_mut {
    ($e:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($e))
    };
}



/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ToolkitApp {
    device_name: Option<String>,

}

impl Default for ToolkitApp {
    fn default() -> Self {
        Self {
            device_name: None,
        }
    }
}

impl ToolkitApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn set_evergreen(&mut self, mut evergreen: Evergreen) {
        evergreen.add_consumer(||Box::new(Session::new()));
        //add more consumers here

        std::thread::spawn(move ||{
            evergreen.do_loop();
        });
        
    }
}



impl eframe::App for ToolkitApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { ..} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });

                Grid::new("erm").show(ui, |ui| {
                    ui.label("Source");
                    ui.label("Destination");
                    ui.label("Protocol");
                    ui.label("Length");
                    ui.label("Info");
                    ui.end_row();
                    // for packet in self.packets.iter() {
                    //     ui.label(packet.source);
                    //     ui.label(packet.destination);
                    //     ui.label(packet.protocol);
                    //     ui.label(packet.length);
                    //     ui.label(packet.info);
                    //     ui.end_row();
                    // }
                });
                ui.menu_button("Network", |ui| {
                    ui.menu_button("Choose Network Interface", |ui|{
                        let devices_res = Device::list();
                        if let Ok(devices) = devices_res {
                            for device in devices.iter() {
                                let desc = format!("{}", device.desc.clone().unwrap_or(device.name.clone()));
                                if ui.button(desc.clone()).clicked() {
                                    self.set_evergreen(Evergreen::new(device.clone()));
                                    self.device_name = Some(desc);
                                    ui.close_menu();
                                }
                            }
                        }
                    })
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });

            // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}