use gtk::prelude::*;

pub use gdk::Display;
pub use gtk::{CssProvider, StyleContext};
pub use crate::gui::{load_css};

mod gui;
mod functions;
mod constants;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    // application.connect_startup(|_| load_css());
    application.connect_activate(gui::build_ui);
    application.run();
}