mod links;
mod navigation;
mod render;
mod search;
mod theme;
mod viewer;

use std::env;
use std::path::PathBuf;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "io.github.marnunez.mdv";

fn main() {
    let file_path = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        eprintln!("Usage: mdv <path/to/file.md>");
        std::process::exit(1);
    });

    if !file_path.exists() {
        eprintln!("File not found: {}", file_path.display());
        std::process::exit(1);
    }

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        viewer::build_ui(app, file_path.clone());
    });

    app.run_with_args(&["mdv"]);
}
