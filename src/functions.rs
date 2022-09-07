use std::cell::Cell;
use std::fs::{self};
use std::rc::Rc;
use std::io::Write;

// use crate::constants::*;

pub fn file_writer(char: String, equals: bool, clear: bool) -> String {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();
    file.write_all(char.as_bytes());

    let y: f64 = if equals == true { 
        let content = std::fs::read_to_string("data.txt").expect("Read failed");
        let v: Vec<&str> = content.split(' ').collect();
        println!("VECTOR: {:?}", v);
        let operand = v[1].parse::<char>().unwrap();
        
        match operand {
            '+' => v[0].parse::<f64>().unwrap() + v[2].parse::<f64>().unwrap(),
            '-' => v[0].parse::<f64>().unwrap() - v[2].parse::<f64>().unwrap(),
            '*' => v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap(),
            '/' => v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap(),
            _ => 0.0,
        }
    } else { 0.0 };
    let a = y.to_string();
    println!("RESULT = {}", a);
    a
}

pub fn set_value(num_counter: i32, dot_counter: i32, value_1: &Rc<Cell<f64>>, value_2: &Rc<Cell<f64>>, num: f64) {
    if dot_counter == 0 {
        println!("-> dot_counter == 0");
        if num_counter == 0 {
            value_1.set(value_1.get() * 10.0 + num);
        }
        if num_counter == 1 {
            value_2.set(value_2.get() * 10.0 + num);
        }
    }
    if dot_counter == 1 {
        println!("-> dot_counter == 1");
        if num_counter == 0 {
            value_1.set(value_1.get() + num / 10.0);
        }
        if num_counter == 1 {
            value_2.set(value_2.get() * 10.0 + num);
        }
    }
    if dot_counter == 2 {
        println!("-> dot_counter == 2");
        if num_counter == 0 {
            value_1.set(value_1.get() * 10.0 + num);
        }
        if num_counter == 1 {
            value_2.set(value_2.get() + num / 10.0);
        }
    }
}
/* 
pub fn the_result(current_operation: char, value_1: &Rc<Cell<f64>>, value_2: &Rc<Cell<f64>>) -> std::string::String {
    let mut result = String::from(" = ");
    // Add operation symbol to variable
    let operation_symbol = match current_operation {
        ADD      => " + ",
        SUBTRACT => " - ",
        MULTIPLY => " \u{00D7} ",
        DIVIDE   => " \u{00F7} ",
        _ => "Error"
    };

    let operation_string = format!("{}{}{}", value_1.get(), operation_symbol, value_2.get());

    match current_operation {
        ADD      => { value_1.set(value_1.get() + value_2.get()); },
        SUBTRACT => { value_1.set(value_1.get() - value_2.get()); },
        MULTIPLY => { value_1.set(value_1.get() * value_2.get()); },
        DIVIDE   => { value_1.set(value_1.get() / value_2.get()); },
        _ => ()
    }
    if current_operation == DIVIDE && value_2.get() == 0.0 {
        result =  String::from("Error: divide by 0");
    }
    else {
        result.push_str( &value_1.get().to_string() );
        result = format!("{}{} ", operation_string, result)
    }

    result
}
*/