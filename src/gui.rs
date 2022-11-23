// #![allow(unused)]

use gtk::Entry;
use std::cell::Cell;
use std::rc::Rc;
use gtk::prelude::*;
use glib_macros::clone;

// If there are no other chars and "-" is the first, type it with no spaces
// If there are first number type " - "
// if there are first number and any operand, type "-" with no spaces

use crate::functions::entry_parser;
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
    let value_1: Rc<Cell<u8>> = Rc::new(Cell::new(NONE));
    let operand: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let entry = Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();
    grid.attach(&entry, 0, 0, 3 ,1);

    // NUM BUTTONS
    for iterator in 0..=9 {
        let button = gtk::Button::with_label(&iterator.to_string());
        let mut column = 0;
        let mut raw = 1;

        button.connect_clicked(clone!( @strong entry, @strong value_1 => move |_| {
                if value_1.take() == 0 {
                    value_1.set(1)
                } else {
                    value_1.set(2)
                }
                // println!("{}", value_1.take());
                entry.insert_text(&iterator.to_string(), &mut -1);
            }));

        if iterator == 1 || iterator == 4 || iterator == 7 { column = 0 }
        if iterator == 2 || iterator == 5 || iterator == 8 { column = 1 }
        if iterator == 3 || iterator == 6 || iterator == 9 { column = 2 }
        if iterator == 0 { column = 1 }
        
        if      (1..=3).contains(&iterator) { raw = 1 }
        else if (4..=6).contains(&iterator) { raw = 2 }
        else if (7..=9).contains(&iterator) { raw = 3 }
        else if iterator == 0                     { raw = 4 }

        grid.attach(&button, column, raw, 1, 1);
    }

    // --> OPERATORS
    let plus_button  = gtk::Button::with_label(ADD);
    let minus_button = gtk::Button::with_label(SUBTRACT);
    let mult_button  = gtk::Button::with_label(MULTIPLY);
    let div_button   = gtk::Button::with_label(DIVIDE);
    let equals_bttn  = gtk::Button::with_label(EQUALS);
    // --> EXTRA BUTTONS
    let dot_button   = gtk::Button::with_label(".");
    let clear_button = gtk::Button::with_label("C");

    // --> CONNECT FUNCTION TO OPERATOR
    plus_button.connect_clicked(clone!(@strong entry, @strong operand => move |_| {
            operand.set(true);
            entry.insert_text(ADD, &mut -1);
        }));
    minus_button.connect_clicked(clone!(@strong entry, @strong value_1, @strong operand => move |_| {
            if value_1.take() == 0 || value_1.take() == 2 || operand.take() == true {
                entry.insert_text(NEGATIVE, &mut -1)
            } else if value_1.take() == 1 {
                operand.set(true);
                entry.insert_text(SUBTRACT, &mut -1)
            }
            println!("{}", value_1.take());
        }));

    mult_button.connect_clicked(clone!(@strong entry, @strong operand => move |_| {
            operand.set(true);
            entry.insert_text(MULTIPLY, &mut -1);
        }));

    div_button.connect_clicked(clone!(@strong entry, @strong operand => move |_| {
            operand.set(true);
            entry.insert_text(DIVIDE, &mut -1);
        }));

    equals_bttn.connect_clicked(clone!(@strong entry, @strong operand => move |_| {
        let get_entry = entry.text();
        let entry_data: String = format!("{}", get_entry);
        let result = entry_parser(entry_data.clone());
        let entry_vew = format!("{} = {}", entry_data, result);

        operand.set(false);
        entry.set_text(&entry_vew);
        }));

    dot_button.connect_clicked(clone!(@strong entry, => move |_| {
            entry.insert_text(".", &mut -1);
        }));

    clear_button.connect_clicked(clone!(@strong entry => move |_| {
            value_1.set(NONE);
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