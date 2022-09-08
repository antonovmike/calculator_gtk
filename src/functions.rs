use std::fs::{self};
use std::io::Write;

pub fn file_writer(char: String, equals: bool) -> String {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();
    file.write_all(char.as_bytes());

    let result: String = if equals == true { 
        let content = std::fs::read_to_string("data.txt").expect("Read failed");
        let v: Vec<&str> = content.split(' ').collect();
        println!("VECTOR: {:?}", v);
        let operand = v[1].parse::<char>().unwrap();
        
        match operand {
            '+' => (v[0].parse::<f64>().unwrap() + v[2].parse::<f64>().unwrap()).to_string(),
            '-' => (v[0].parse::<f64>().unwrap() - v[2].parse::<f64>().unwrap()).to_string(),
            '*' => format!("{:.6}", v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap()),
            '/' => format!("{:.6}", v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap()),
            _ => "0".to_string(),
        }
    } else { "0".to_string() };
    result
}
