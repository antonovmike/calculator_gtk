use std::cell::Cell;
use std::fs::{self};
use std::rc::Rc;
use std::io::Write;

pub fn file_writer(char: String, equals: bool, _clear: bool) -> String {
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