// #[allow(dead_code)]

pub fn entry_parser(entry_data: String) -> String {
    if entry_data.len() != 0 {
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
