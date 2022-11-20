use gtk::prelude::*;

pub use gdk::Display;
pub use gtk::{CssProvider, StyleContext};

mod constants;
mod functions;
mod gui;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    application.connect_activate(gui::build_ui);
    application.run();
}
