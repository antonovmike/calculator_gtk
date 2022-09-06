use gtk::prelude::*;
use std::fs;
use std::path::Path;

pub use gdk::Display;
pub use gtk::{CssProvider, StyleContext};
// pub use crate::gui::{load_css};
// pub use gtk::{gdk, Application, ApplicationWindow, Button, CssProvider, StyleContext};

mod gui;
mod functions;
mod constants;

fn main() {
    if Path::new("data.txt").exists() {
        fs::remove_file("data.txt").unwrap();
    }

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    // application.connect_startup(|_| load_css());
    application.connect_activate(gui::build_ui);
    application.run();
}