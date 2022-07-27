use gtk::prelude::*;

mod gui;
mod functions;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    application.connect_activate(gui::build_ui);
    application.run();
}