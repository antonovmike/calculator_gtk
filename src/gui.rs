use gtk::Entry;
use std::cell::Cell;
use std::rc::Rc;
use gtk::prelude::*;
use glib_macros::clone;

use crate::functions::{the_result, set_value};
use crate::constants::*;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("GTK calc");
    window.set_default_size(200, 120);

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

    // NUM BUTTONS
    let mut button_value = 1.0;
    for iterator in 1..=9 {
        let button = gtk::Button::with_label(&iterator.to_string());
        let mut column = 0;
        let mut raw = 1;

        button.connect_clicked(clone!(
            @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
            @strong dot_counter, @strong  value_1_temp =>
            move |_| {
                set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, button_value);
                entry.insert_text(&iterator.to_string(), &mut -1);
            }));
        
        if iterator % 3 == 1 {
            column = 0;
        } else if iterator % 3 == 2 {
            column = 1;
        } else if iterator % 3 == 0 {
            column = 2;
        }
        if iterator > 3 && iterator < 7 { raw = 2 }
        else if iterator >= 7 && iterator <= 9 { raw = 3 }

        grid.attach(&button, column, raw, 1, 1);

        button_value += 1.0;
    }
    
    let button_0 = gtk::Button::with_label("0");
    button_0.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry,
        @strong dot_counter, @strong  value_1_temp =>
        move |_| {
            set_value(num_counter.get(), dot_counter.get(), &value_1, &value_2, 0.0);
            entry.insert_text("0", &mut -1);
            println!("{}{}", "\u{00D7}", "\u{00F7}");
        }));
    grid.attach(&button_0, 1, 4, 1, 1);

    // --> OPERATORS
    let plus_button  = gtk::Button::with_label(" + ");
    let minus_button = gtk::Button::with_label(" - ");
    let mult_button  = gtk::Button::with_label(" × ");
    let div_button   = gtk::Button::with_label(" ÷ ");
    let equals_bttn  = gtk::Button::with_label(" = ");
    // --> EXTRA BUTTONS
    let dot_button   = gtk::Button::with_label("."); // FIX IT
    let clear_button = gtk::Button::with_label("C");
    // clear_button.add_css_class("button");

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

                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0); // Reset to 0
            }
            else {
                current_operation.set(ADD);
            }
            entry.insert_text(" + ", &mut -1);
        }));
    minus_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, 
        @strong previous_operation, @strong current_operation =>
        move |_| {
            num_counter.set(num_counter.get() + 1);
            if num_counter.get() == 2 {
                previous_operation.set(current_operation.get());
                current_operation.set(SUBTRACT);

                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0); // Reset to 0
            }
            else {
                current_operation.set(SUBTRACT);
            }
            entry.insert_text(" - ", &mut -1);
        }));

    mult_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, 
        @strong previous_operation, @strong current_operation =>
        move |_| {
            num_counter.set(num_counter.get() + 1);
            if num_counter.get() == 2 {
                previous_operation.set(current_operation.get());
                current_operation.set(MULTIPLY);

                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0);
            }
            else {
                current_operation.set(MULTIPLY);
            }
            entry.insert_text(" \u{00D7} ", &mut -1);
        }));

    div_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry, 
        @strong previous_operation, @strong current_operation =>
        move |_| {
            num_counter.set(num_counter.get() + 1);
        
            if num_counter.get() == 2 {
                previous_operation.set(current_operation.get());
                current_operation.set(DIVIDE);

                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0);
            }
            else {
                current_operation.set(DIVIDE);
            }
            entry.insert_text(" \u{00F7} ", &mut -1);
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

    // --> FIX IT <--
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