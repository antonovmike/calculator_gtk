use gtk::Entry;
use std::cell::Cell;
use std::rc::Rc;
use gtk::prelude::*;
use glib_macros::clone;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("GTK calc");
    window.set_default_size(200, 120);

    // Construct the grid that is going contain buttons
    let margin = 6;
    let grid = gtk::Grid::builder()
        .margin_start(margin)
        .margin_end(margin)
        .margin_top(margin)
        .margin_bottom(margin)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(margin)
        .column_spacing(margin)
        .build();

    window.set_child(Some(&grid));

    let val1: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let val2: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let num_counter = Rc::new(Cell::new(0));
    let entry = Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();
    grid.attach(&entry, 0, 0, 4 ,1);

    let button_1 = gtk::Button::with_label("1");
    let button_1 = gtk::Button::with_label("2");
    let button_1 = gtk::Button::with_label("3");
    let button_1 = gtk::Button::with_label("4");
    let button_1 = gtk::Button::with_label("5");
    let button_1 = gtk::Button::with_label("6");
    let button_1 = gtk::Button::with_label("7");
    let button_1 = gtk::Button::with_label("8");
    let button_1 = gtk::Button::with_label("9");
    let button_1 = gtk::Button::with_label("0");

    button_1.connect_clicked(clone!(@strong val1, @strong val2, @strong num_counter, 
        @strong entry =>
        move |_| {
            entry.insert_text("1", &mut -1);
        }));

    grid.attach(&button_1, 0, 1, 1, 1);
    grid.attach(&button_2, 1, 1, 1, 1);
    grid.attach(&button_3, 2, 1, 1, 1);
    grid.attach(&button_4, 0, 2, 1, 1);
    grid.attach(&button_5, 1, 2, 1, 1);
    grid.attach(&button_6, 2, 2, 1, 1);
    grid.attach(&button_7, 0, 3, 1, 1);
    grid.attach(&button_8, 1, 3, 1, 1);
    grid.attach(&button_9, 2, 3, 1, 1);
    grid.attach(&button_0, 0, 4, 3, 1);


    // --> OPERATORS
    let plus_button  = gtk::Button::with_label("+");
    let minus_button = gtk::Button::with_label("-");
    let mult_button  = gtk::Button::with_label("\u{00D7}");
    let div_button   = gtk::Button::with_label("\u{00F7}");

    plus_button.connect_clicked(glib::clone!(@weak entry => move |_| {
        let nb = entry.text()
            .parse()
            .unwrap_or(0.0);
            entry.set_text(&format!("{}", nb + 1.1));
    }));
    minus_button.connect_clicked(glib::clone!(@weak entry => move |_| {
        let nb = entry.text()
            .parse()
            .unwrap_or(0.0);
            entry.set_text(&format!("{}", nb - 1.2));
    }));
    mult_button.connect_clicked(glib::clone!(@weak entry => move |_| {
        let nb = entry.text()
            .parse()
            .unwrap_or(0.0);
            entry.set_text(&format!("{}", nb * 1.3));
    }));
    div_button.connect_clicked(glib::clone!(@weak entry => move |_| {
        let nb = entry.text()
            .parse()
            .unwrap_or(0.0);
            entry.set_text(&format!("{}", nb / 1.4));
    }));

    grid.attach(&plus_button,  3, 1, 1, 1);
    grid.attach(&minus_button, 3, 2, 1, 1);
    grid.attach(&mult_button,  3, 3, 1, 1);
    grid.attach(&div_button,   3, 4, 1, 1);

    window.show_all();
}