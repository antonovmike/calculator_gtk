// use std::io;
// use gtk::prelude::*;

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     let vector: Vec<char> = input.chars().collect();
//     let first_number = (vector[0].to_string()).parse::<f32>().unwrap();
//     let operator = vector[1];
//     let second_number = (vector[2].to_string()).parse::<f32>().unwrap();

//     let result = operate(operator, first_number, second_number);

//     println!(
//         "{:?}",
//         output(first_number, operator, second_number, result)
//     );
// }

// fn operate(operator: char, first_number: f32, second_number: f32) -> String {
//     match operator {
//         '+' => (first_number + second_number).to_string(),
//         '-' => (first_number - second_number).to_string(),
//         '/' => (first_number / second_number).to_string(),
//         '*' | 'x' | 'X' => (first_number * second_number).to_string(),
//                _ => ("Invalid operator used.").to_string()
//     }
// }

// fn output(first_number: f32, operator: char, second_number: f32, result: String) -> String {
//     format!(
//         "{} {} {} = {}",
//         first_number, operator, second_number, result
//     )
// }

use gtk::prelude::*;

mod gui;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    application.connect_activate(gui::build_ui);
    application.run();
}