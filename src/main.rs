use gtk::prelude::*;

mod constants;
mod functions;
mod gui;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.calculator_gtk"),
        Default::default(),
    );

    application.connect_activate(gui::build_ui);
    application.run();
}
