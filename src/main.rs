use gtk::prelude::*;

pub use gdk::Screen;
pub use gtk::{CssProvider, StyleContext};

mod gui;
mod functions;
mod constants;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    application.connect_activate(gui::build_ui);
    application.run();
}