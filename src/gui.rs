use gtk::Entry;
use std::cell::Cell;
use std::rc::Rc;
use gtk::prelude::*;
use glib_macros::clone;

use crate::functions::{operation, the_result, set_value};

pub const ADD: char = '+';
pub const SUBTRACT: char = '-';
pub const MULTIPLY: char = '*';
pub const DIVIDE: char = '/';
pub const EQUALS: char = '=';
pub const NONE: char = '0';

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("GTK calc");
    window.set_default_size(200, 120);

    // --> CREATE GRID
    let margin = 5;
    let grid = gtk::Grid::builder()
        .margin_start(margin)
        .margin_end(margin)
        .margin_top(margin)
        .margin_bottom(margin)
        .row_spacing(margin)
        .column_spacing(margin)
        .build();

    window.set_child(Some(&grid));

    // --> OPERATIONAL DATA
    let value_1: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let value_2: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));

    let dot_detector: Rc<Cell<char>> = Rc::new(Cell::new('_'));
    let value_1_temp: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let num_counter = Rc::new(Cell::new(0));
    let dot_counter = Rc::new(Cell::new(0)); // INCREASES EACH TIME dot_button PRESSED
    let previous_operation = Rc::new(Cell::new(NONE));
    let current_operation = Rc::new(Cell::new(NONE));
    let entry = Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();
    grid.attach(&entry, 0, 0, 3 ,1);

    // --> CREATE NUM BUTTONS
    let button_1 = gtk::Button::with_label("1");
    let button_2 = gtk::Button::with_label("2");
    let button_3 = gtk::Button::with_label("3");
    let button_4 = gtk::Button::with_label("4");
    let button_5 = gtk::Button::with_label("5");
    let button_6 = gtk::Button::with_label("6");
    let button_7 = gtk::Button::with_label("7");
    let button_8 = gtk::Button::with_label("8");
    let button_9 = gtk::Button::with_label("9");
    let button_0 = gtk::Button::with_label("0");

    // --> CONNECT FUNCTION
    button_1.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 1.0);
            entry.insert_text("1", &mut -1);
        }));
        
    button_2.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 2.0);
            entry.insert_text("2", &mut -1);
        }));
    button_3.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 3.0);
            entry.insert_text("3", &mut -1);
        }));
    button_4.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 4.0);
            entry.insert_text("4", &mut -1);
        }));
    button_5.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 5.0);
            entry.insert_text("5", &mut -1);
        }));
    button_6.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 6.0);
            entry.insert_text("6", &mut -1);
        }));
    button_7.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 7.0);
            entry.insert_text("7", &mut -1);
        }));
    button_8.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 8.0);
            entry.insert_text("8", &mut -1);
        }));
    button_9.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 9.0);
            entry.insert_text("9", &mut -1);
        }));
    button_0.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 0.0);
            entry.insert_text("0", &mut -1);
        }));

    // --> ATTACH NUM BUTTONS TO GRID
    grid.attach(&button_1, 0, 1, 1, 1);
    grid.attach(&button_2, 1, 1, 1, 1);
    grid.attach(&button_3, 2, 1, 1, 1);
    grid.attach(&button_4, 0, 2, 1, 1);
    grid.attach(&button_5, 1, 2, 1, 1);
    grid.attach(&button_6, 2, 2, 1, 1);
    grid.attach(&button_7, 0, 3, 1, 1);
    grid.attach(&button_8, 1, 3, 1, 1);
    grid.attach(&button_9, 2, 3, 1, 1);
    grid.attach(&button_0, 1, 4, 1, 1);

    // --> OPERATORS
    let plus_button  = gtk::Button::with_label("+");
    let minus_button = gtk::Button::with_label("-");
    let mult_button  = gtk::Button::with_label("\u{00D7}");
    let div_button   = gtk::Button::with_label("\u{00F7}");
    let equals_bttn  = gtk::Button::with_label("=");
    // --> EXTRA BUTTONS
    let dot_button   = gtk::Button::with_label("."); // FIX IT
    let clear_button = gtk::Button::with_label("C");

    // --> CONNECT FUNCTION TO OPERATOR
    plus_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, 
        @strong previous_operation, @strong current_operation =>
        move |_| {
            // Increase num_counter
            num_counter.set(num_counter.get() + 1);
            // After second number has been inserted
            if num_counter.get() == 2 {
                // Set previous and current operation
                previous_operation.set(current_operation.get());
                current_operation.set(ADD);

                operation(previous_operation.get(), &value_1, &value_2);

                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0); // Reset to 0
            }
            else {
                current_operation.set(ADD);
            }
            entry.insert_text("+", &mut -1);
        }));
    minus_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, 
        @strong previous_operation, @strong current_operation =>
        move |_| {
            num_counter.set(num_counter.get() + 1);
            if num_counter.get() == 2 {
                previous_operation.set(current_operation.get());
                current_operation.set(SUBTRACT);
                operation(previous_operation.get(), &value_1, &value_2);
                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0); // Reset to 0
            }
            else {
                current_operation.set(SUBTRACT);
            }
            entry.insert_text("-", &mut -1);
        }));

    mult_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, 
        @strong previous_operation, @strong current_operation =>
        move |_| {
            num_counter.set(num_counter.get() + 1);
            if num_counter.get() == 2 {
                previous_operation.set(current_operation.get());
                current_operation.set(MULTIPLY);
                operation(previous_operation.get(), &value_1, &value_2);
                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0);
            }
            else {
                current_operation.set(MULTIPLY);
            }
            entry.insert_text("\u{00D7}", &mut -1);
        }));

    div_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, 
        @strong previous_operation, @strong current_operation =>
        move |_| {
            num_counter.set(num_counter.get() + 1);
        
            if num_counter.get() == 2 {
                previous_operation.set(current_operation.get());
                current_operation.set(DIVIDE);
                operation(previous_operation.get(), &value_1, &value_2);
                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0);
            }
            else {
                current_operation.set(DIVIDE);
            }
            entry.insert_text("\u{00F7}", &mut -1);
        }));

    equals_bttn.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong current_operation, @strong entry, @strong dot_counter =>
        move |_| {
            // Increase num_counter
            num_counter.set(num_counter.get() + 1);
            // After second number has been inserted
            if num_counter.get() == 2 {
                let result = the_result(current_operation.get(), &value_1, &value_2);

                entry.set_text(&result);
                previous_operation.set(EQUALS);

                // Reset variables
                num_counter.set(0);
                dot_counter.set(0);
                value_1.set(0.0);
                value_2.set(0.0);
                current_operation.set(NONE);
            }
        }));

    dot_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, @strong dot_counter =>
        move |_| {
            // dot_counter.set(dot_counter.get() + 1);
            if dot_counter.get() == 0 {
                println!("dot_counter inside dot button IF: {}", dot_counter.get());
                dot_counter.set(dot_counter.get() + 1);
                set_value(num_counter.get(), 1, &value_1, &value_2, 0.0);
            } else if dot_counter.get() == 1 {
                println!("dot_counter inside dot button ELSE: {}", dot_counter.get());
                dot_counter.set(dot_counter.get() + 1);
                set_value(num_counter.get(), 2, &value_1, &value_2, 0.0);
            } else {
                dot_counter.set(0);
            }
            entry.insert_text(".", &mut -1); // Add '.' to entry
        }));

    clear_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, @strong dot_detector, @strong value_1_temp =>
        move |_| {
            // CLEAR
            num_counter.set(0);
            value_1.set(0.0);
            value_2.set(0.0);
            dot_counter.set(0);
            entry.set_text("");
        }));

    // --> ATTACH OPERATORS AND EXTRA BUTTONS TO GRID
    grid.attach(&plus_button,  3, 1, 1, 1);
    grid.attach(&minus_button, 3, 2, 1, 1);
    grid.attach(&mult_button,  3, 3, 1, 1);
    grid.attach(&div_button,   3, 4, 1, 1);
    grid.attach(&equals_bttn,  2, 4, 1, 1);
    grid.attach(&dot_button,   0, 4, 1, 1);
    grid.attach(&clear_button, 3, 0, 1, 1);

    window.show_all();
}