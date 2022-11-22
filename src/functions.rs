pub fn entry_parser(entry_data: String) -> String {
    if entry_data.len() != 0 {
        let v: Vec<&str> = entry_data.split(' ').collect();
        println!("VECTOR: {:?}", v);
        let operand = v[1].parse::<char>().unwrap();
        println!("operand = {}", operand);

        match operand {
            '+' => (v[0].parse::<f64>().unwrap() + v[2].parse::<f64>().unwrap()).to_string(),
            '-' => (v[0].parse::<f64>().unwrap() - v[2].parse::<f64>().unwrap()).to_string(),
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
    println!("{:?}", vec);
    let string_1: String = vec.iter().collect::<String>();
    string_1
}

#[test]
fn empty() {
    let entry_data = "".to_string();
    assert_eq!("0", &entry_parser(entry_data))
}
#[test]
fn add() {
    let entry_data = "1 + 3".to_string();
    assert_eq!("4", &entry_parser(entry_data))
}
#[test]
fn sub() {
    let entry_data = "1.022 - 3.009".to_string();
    assert_eq!("-1.9869999999999999", &entry_parser(entry_data))
}
// #[test]
// fn mul() {}
// #[test]
// fn div() {}
