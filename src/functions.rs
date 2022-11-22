// #[allow(dead_code)]

pub fn entry_parser(entry_data: String) -> String {
    // Check if Entry is sempty string 
    // Check if only one number 
    // Check if Entry contains only letters
    // Add char + digit "a1" checker
    // Check if first number and operand with no second number
    if entry_data.len() != 0 
    && entry_data.chars().all(|x| x.is_alphanumeric()) == false 
    // && entry_data.chars().all(|x| x.is_ascii_alphabetic())
    && entry_data.chars().last().unwrap().is_numeric() {
        let v: Vec<&str> = entry_data.split(' ').collect();
        let operand = v[1].parse::<char>().unwrap();

        match operand {
            '+' => (v[0].parse::<f64>().unwrap() + v[2].parse::<f64>().unwrap()).to_string(),
            '-' => (v[0].parse::<f64>().unwrap() - v[2].parse::<f64>().unwrap()).to_string(),
            // '×' => format!("{:.6}", v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap()),
            // '÷' => format!("{:.6}", v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap()),
            '×' => format!(
                "{:.6}",
                extra_zeroes_remover(v[0].parse::<f64>().unwrap() * v[2].parse::<f64>().unwrap())
            ),
            '÷' => format!(
                "{:.6}",
                extra_zeroes_remover(v[0].parse::<f64>().unwrap() / v[2].parse::<f64>().unwrap())
            ),
            _ => "0".to_string(),
        }
    } else {
        "0".to_string()
    }
}

fn extra_zeroes_remover(f: f64) -> String {
    let string_0 = format!("{}", f);
    let mut vec: Vec<char> = string_0.chars().collect();
    for _i in 0..vec.len() {
        let last_element = vec[vec.len() - 1];
        if last_element == '0' {
            vec.remove(vec.len() - 1);
        }
    }
    
    vec.iter().collect::<String>()
}
