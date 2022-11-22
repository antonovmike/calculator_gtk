pub fn entry_parser(entry_data: String, equals: bool) -> String {
    if equals {
        let v: Vec<&str> = entry_data.split(' ').collect();
        println!("VECTOR: {:?}", v);
        let operand = v[1].parse::<char>().unwrap();
        println!("operand = {}", operand);
        
        match operand {
            '+' => (v[0].parse::<f64>().unwrap() + v[2].parse::<f64>().unwrap()).to_string(),
            '-' => (v[0].parse::<f64>().unwrap() - v[2].parse::<f64>().unwrap()).to_string(),
            '×' => format!("{:.6}", extra_zeroes_remover(v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap())),
            '÷' => format!("{:.6}", extra_zeroes_remover(v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap())),
            _ => "0".to_string(),
        }
    } else { "0".to_string() }
}

fn extra_zeroes_remover(f: f64) -> String {
    let string0 = format!("{}", f);
    let mut vec: Vec<char> = string0.chars().collect();
    for _i in 0..vec.len() {
        let last_element = vec[vec.len() - 1];
        if last_element == '0' {
            vec.remove(vec.len() - 1);
        }
    }
    println!("{:?}", vec);
    let string1: String = vec.iter().collect::<String>();
    string1
}

// #[test]
// fn add() {}
// #[test]
// fn min() {}
// #[test]
// fn mul() {}
// #[test]
// fn div() {}