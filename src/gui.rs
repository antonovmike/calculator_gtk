use gtk::Entry;
use std::cell::Cell;
use std::rc::Rc;
use gtk::prelude::*;
use glib_macros::clone;

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
    let num_counter = Rc::new(Cell::new(0));
    let previous_operation = Rc::new(Cell::new(NONE));
    let current_operation = Rc::new(Cell::new(NONE));
    let entry = Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();
    grid.attach(&entry, 0, 0, 4 ,1);

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
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 1.0);
            entry.insert_text("1", &mut -1);
        }));
    button_2.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 2.0);
            entry.insert_text("2", &mut -1);
        }));
    button_3.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 3.0);
            entry.insert_text("3", &mut -1);
        }));
    button_4.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 4.0);
            entry.insert_text("4", &mut -1);
        }));
    button_5.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 5.0);
            entry.insert_text("5", &mut -1);
        }));
    button_6.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 6.0);
            entry.insert_text("6", &mut -1);
        }));
    button_7.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 7.0);
            entry.insert_text("7", &mut -1);
        }));
    button_8.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 8.0);
            entry.insert_text("8", &mut -1);
        }));
    button_9.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 9.0);
            entry.insert_text("9", &mut -1);
        }));
    button_0.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong previous_operation, @strong entry =>
        move |_| {
            set_value(num_counter.get(), &value_1, &value_2, 0.0);
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
    let dot_button   = gtk::Button::with_label("."); // FIX IT

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

                operation(previous_operation.get(), &value_1, value_2.get());

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
                operation(previous_operation.get(), &value_1, value_2.get());
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
                operation(previous_operation.get(), &value_1, value_2.get());
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
                operation(previous_operation.get(), &value_1, value_2.get());
                num_counter.set(num_counter.get() - 1);
                value_2.set(0.0);
            }
            else {
                current_operation.set(DIVIDE);
            }
            entry.insert_text("\u{00F7}", &mut -1);
        }));

    equals_bttn.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong current_operation, @strong entry =>
        move |_| {
            // Increase num_counter
            num_counter.set(num_counter.get() + 1);
            // After second number has been inserted
            if num_counter.get() == 2 {
                let result = the_result(current_operation.get(), &value_1, value_2.get());

                entry.set_text(&result);
                previous_operation.set(EQUALS);

                // Reset variables
                num_counter.set(0);
                value_1.set(0.0);
                value_2.set(0.0);
                current_operation.set(NONE);
            }
        }));

    // --> FIX IT <--
    // --> FIX IT <--
    // --> FIX IT <--
    dot_button.connect_clicked(clone!(
        @strong value_1, @strong value_2, @strong num_counter, @strong entry =>
        move |_| {
            // Add the '.' to value
            set_value(num_counter.get(), &value_1, &value_2, 0.0); // Turns '.' into 0. "0.05" would turn into 5, "1.01" - into 105
            entry.insert_text(".", &mut -1); // Doesn't work yet
        }));

    // --> ATTACH OPERATORS TO GRID
    grid.attach(&plus_button,  3, 1, 1, 1);
    grid.attach(&minus_button, 3, 2, 1, 1);
    grid.attach(&mult_button,  3, 3, 1, 1);
    grid.attach(&div_button,   3, 4, 1, 1);
    grid.attach(&equals_bttn,  2, 4, 1, 1);
    grid.attach(&dot_button,   0, 4, 1, 1);

    window.show_all();
}

pub fn set_value(num_counter: i32, value_1: &Rc<Cell<f64>>, value_2: &Rc<Cell<f64>>, num: f64) {
    if num_counter == 0 {
        value_1.set(value_1.get() * 10.0 + num);
    }
    if num_counter == 1 {
        value_2.set(value_2.get() * 10.0 + num);
    }
}

pub fn operation(previous_operation: char, value_1: &Rc<Cell<f64>>, value_2: f64) {
    match previous_operation {
        ADD      => value_1.set(value_1.get() + value_2),
        SUBTRACT => value_1.set(value_1.get() - value_2),
        MULTIPLY => value_1.set(value_1.get() * value_2),
        DIVIDE   => value_1.set(value_1.get() / value_2),
        _=> ()
    }
}

fn the_result(current_operation: char, value_1: &Rc<Cell<f64>>, value_2: f64) -> std::string::String {
    let mut result = String::from(" = ");

    let operation_symbol = match current_operation {
        ADD =>      "+",
        SUBTRACT => "-",
        MULTIPLY => "\u{00D7}",
        DIVIDE =>   "\u{00F7}",
        _=> "_"
    };

    let operation_string = format!("{}{}{}", value_1.get(), operation_symbol, value_2);
    // println!("operation_string: {}", operation_string);
    // dbg!("{:?}", &result);
    match current_operation {
        ADD =>      { value_1.set(value_1.get() + value_2); },
        SUBTRACT => { value_1.set(value_1.get() - value_2); },
        MULTIPLY => { value_1.set(value_1.get() * value_2); },
        DIVIDE =>   { value_1.set(value_1.get() / value_2); },
        _=> ()
    }
    if current_operation == DIVIDE && value_2 == 0.0 {
        result =  String::from("Error: divide by 0");
    }
    else {
        result.push_str( &value_1.get().to_string() );
        result = format!("{}{}", operation_string, result)
    }
    // dbg!("{:?}", &result);
    result
}