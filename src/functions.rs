use crate::constants::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse float value")]
    FloatError(#[from] std::num::ParseFloatError),

    #[error("Failed to parse float value")]
    CharError(#[from] std::char::ParseCharError),
}

// Does all the math
pub fn entry_parser(entry_data: String) -> Result<String, Error> {
    if char_check(&entry_data) {
        let v: Vec<&str> = entry_data.split(' ').collect();
        let operand = v[1].parse::<char>()?;

        match operand {
            '+' => Ok((v[0].parse::<f64>()? + v[2].parse::<f64>()?).to_string()),
            '-' => Ok((v[0].parse::<f64>()? - v[2].parse::<f64>()?).to_string()),
            '×' => Ok(format!(
                "{:.6}",
                extra_zeroes_remover(v[0].parse::<f64>()? * v[2].parse::<f64>()?)
            )),
            '÷' => Ok(format!(
                "{:.6}",
                extra_zeroes_remover(v[0].parse::<f64>()? / v[2].parse::<f64>()?)
            )),
            _ => Ok("0".to_string()),
        }
    } else {
        Ok("0".to_string())
    }
}
// Remove extra zeroes
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
// Check for wrong input
fn char_check(entry_data: &String) -> bool {
    if 
    entry_data.contains(ADD) || entry_data.contains(SUBTRACT) ||
    entry_data.contains(MULTIPLY) || entry_data.contains(DIVIDE)
    { 
        if entry_data.len() > 4 
        && !entry_data.chars().all(|x| x.is_alphanumeric())
        && entry_data.chars().last().unwrap().is_numeric() {
            true 
        } else { false }        
    } else { false }
}