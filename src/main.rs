use std::io;
use std::fs;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::process::Command;


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {
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


    let mut entry_name = String::new();
    let mut entry_path = String::new();

    //app
    let app = eframe_template::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
