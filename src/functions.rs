#![allow(unused)]

use std::fs::{self};
use std::io::Write;

pub fn entry_parser(entry_data: String, equals: bool) -> String {
    let result: String = if equals == true {
        let v: Vec<&str> = entry_data.split(' ').collect();
        println!("VECTOR: {:?}", v);
        let operand = v[1].parse::<char>().unwrap();
        println!("operand = {}", operand);
        
        match operand {
            '+' => (v[0].parse::<f64>().unwrap() + v[2].parse::<f64>().unwrap()).to_string(),
            '-' => (v[0].parse::<f64>().unwrap() - v[2].parse::<f64>().unwrap()).to_string(),
            '×' => format!("{:.6}", v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap()),
            '÷' => format!("{:.6}", v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap()),
            _ => "0".to_string(),
        }
    } else { "0".to_string() };

    println!("entry_data = {}", entry_data);
    println!("result = {}", result);

    result
}

// #[test]
// fn add() {}
// #[test]
// fn min() {}
// #[test]
// fn mul() {}
// #[test]
// fn div() {}