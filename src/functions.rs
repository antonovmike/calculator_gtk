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
            '*' => format!("{:.6}", v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap()),
            '/' => format!("{:.6}", v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap()),
            _ => "0".to_string(),
        }
    } else { "0".to_string() };

    println!("entry_data = {}", entry_data);
    println!("result = {}", result);

    result
}

// pub fn file_writer(char: String, equals: bool) -> String {
//     let mut file = fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .open("data.txt")
//         .unwrap();
//     file.write_all(char.as_bytes());

//     let result: String = if equals == true { 
//         let content = std::fs::read_to_string("data.txt").expect("Read failed");
//         let v: Vec<&str> = content.split(' ').collect();
//         println!("VECTOR: {:?}", v);
//         let operand = v[1].parse::<char>().unwrap();
        
//         match operand {
//             '+' => (v[0].parse::<f64>().unwrap() + v[2].parse::<f64>().unwrap()).to_string(),
//             '-' => (v[0].parse::<f64>().unwrap() - v[2].parse::<f64>().unwrap()).to_string(),
//             '*' => format!("{:.6}", v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap()),
//             '/' => format!("{:.6}", v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap()),
//             _ => "0".to_string(),
//         }
//     } else { "0".to_string() };

//     result
// }

// TEST

// fn tester(a: String, b: String, c: String) -> String {
//     file_writer(a, false);
//     file_writer(b, false);
//     file_writer(c, false);
//     let d = file_writer("".to_string(), true);
//     println!("{d}");
//     return d
// }

// #[test]
// fn add() {
//     let file = fs::File::create("data.txt");
//     assert_eq!("4".to_string(), tester("2".to_string(), " + ".to_string(), "2".to_string()));
// }
// #[test]
// fn min() {
//     let file = fs::File::create("data.txt");
//     assert_eq!("0".to_string(), tester("2".to_string(), " - ".to_string(), "2".to_string()));
// }
// #[test]
// fn mul() {
//     let file = fs::File::create("data.txt");
//     assert_eq!("6.000000".to_string(), tester("2".to_string(), " * ".to_string(), "3".to_string()));
// }
// #[test]
// fn div() {
//     let file = fs::File::create("data.txt");
//     assert_eq!("2.000000".to_string(), tester("6".to_string(), " / ".to_string(), "3".to_string()));
// }