use eframe::{egui, epi};
use std::io;
use std::fs;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,

    //entries
    entry_name: String,
    entry_path: String,
    entry_to_delete: String,
    entry_search: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // default stuff:
            label: "".to_owned(),
            value: 0.0,
            entry_name: String::new(),
            entry_path: String::new(),
            entry_to_delete: String::new(),
            entry_search: String::new(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Xlaunch"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { label, value, entry_name, entry_path, entry_to_delete, entry_search } = self;


            //entries
            #[derive(Clone, Serialize, Deserialize, Debug)] 
            struct Entry {
                name: String,
                path: String,
                wine: bool,
            }

            let mut entries = Vec::new();
            entries.push(Entry { name: "i".to_string(), path: "o".to_string(), wine: false });

            let mut data = fs::read_to_string("data.json").expect("unable to read file");
            entries = serde_json::from_str(&data).unwrap();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // top pannel: menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                    if ui.button("kill wine").clicked() {
                        let shell = include_str!("killwine.sh");
                        let script = format!(" {}", shell);
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(script)
                            .spawn()
                            .unwrap()
                            .wait();
                    }
                });
            });
        });

        //left side pannel
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("");
            
            //search
            ui.horizontal(|ui| {
                ui.label("search: ");
                ui.text_edit_singleline(entry_search);
            });
            for i in 0..entries.len() {
                if entries[i].name.get(entries[i].name.len() - entries[i].name.len()..entry_search.len()) == Some(entry_search) {
                    if ui.button(entries[i].name.clone()).clicked() {
                        //launching entry
                        if entries[i].wine == false {
                            let shell = include_str!("launch.sh");
                            let rust_var = entries[i].path.clone();
                            let script = format!("VARIABLE={} ; {}", rust_var, shell);
                            std::process::Command::new("sh")
                                .arg("-c")
                                .arg(script)
                                .spawn()
                                .unwrap()
                                .wait();
                        }else{
                            let shell = include_str!("launch_wine.sh");
                            let rust_var = entries[i].path.clone();
                            let script = format!("VARIABLE={} ; {}", rust_var, shell);
                            std::process::Command::new("sh")
                            .arg("-c")
                                .arg(script)
                                .spawn()
                                .unwrap()
                                .wait();  
                        }
                    }
                }
            }
            
        });   

        //central pannel
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("apps");

            //generating one button for each entry
            for k in 0..entries.len() {
                if ui.button(entries[k].name.clone()).clicked() {
                    //launching entry
                    if entries[k].wine == false {
                        let shell = include_str!("launch.sh");
                        let rust_var = entries[k].path.clone();
                        let script = format!("VARIABLE={} ; {}", rust_var, shell);
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(script)
                            .spawn()
                            .unwrap()
                            .wait();
                        }else{
                        let shell = include_str!("launch_wine.sh");
                        let rust_var = entries[k].path.clone();
                        let script = format!("VARIABLE={} ; {}", rust_var, shell);
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(script)
                            .spawn()
                            .unwrap()
                            .wait();  
                        }
                }
            } 

            //right side pannel
            egui::SidePanel::right("side_panel2").show(ctx, |ui| {

                //button to add an entry
                ui.heading("Add entry");

                ui.horizontal(|ui| {
                    ui.label("entry name: ");
                    ui.text_edit_singleline(entry_name);
                });

                ui.horizontal(|ui| {
                    ui.label("entry path: ");
                    ui.text_edit_singleline(entry_path);
                });

                if ui.button("Add").clicked() {
                    let mut entry_type = String::new();
                    entry_type = entry_path[entry_path.len() - 3..entry_path.len()].to_string();
                    if entry_type.to_string() == "exe".to_string() {
                        println!("autodetect detected that wine is necessary to launch this software/game, adding wine: True to entry.");
                        entries.push(Entry { name: entry_name.to_string(), path: entry_path.to_string(), wine: true });
                    }else{
                        println!("autodetect detected that wine is unnecessary to launch this software/game, adding wine: False to entry.");
                        entries.push(Entry { name: entry_name.to_string(), path: entry_path.to_string(), wine: false });
                    }
                    //writing to file
                    let mut serialized_entries =serde_json::to_string(&entries).unwrap();
                    fs::write("data.json", serialized_entries).expect("unable to write file");
                }

                //button to remove an entry
                ui.heading("Remove entry");

                ui.horizontal(|ui| {
                    ui.label("entry name: ");
                    ui.text_edit_singleline(entry_to_delete);
                });

                if ui.button("Remove").clicked() {
                    for i in 0..entries.len() {
                        if entries[i].name.to_string() == entry_to_delete.to_string(){
                            for j in i..entries.len()-1{
                                entries[j] = entries[j+1].clone();
                            };
                            entries.truncate(entries.len()-1);
                        }
                    };
                    //writing to file
                    let mut serialized_entries =serde_json::to_string(&entries).unwrap();
                    fs::write("data.json", serialized_entries).expect("unable to write file");
                } 
            });
                        
        });


        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}