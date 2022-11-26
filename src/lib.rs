pub mod functions;
pub mod constants;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn wrong_input() {
    let entry_data = "-".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "1 -".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "1".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = ".".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "stop_war".to_string();
    assert_eq!("0", functions::entry_parser(entry_data));
    let entry_data = "abcd1234".to_string();
    assert_eq!("0", functions::entry_parser(entry_data))
}

#[test]
fn add() {
    let entry_data = "1.022 + 3.009".to_string();
    assert_eq!("4.031", &functions::entry_parser(entry_data));
    let entry_data = "-1 + -2".to_string();
    assert_eq!("-3", &functions::entry_parser(entry_data))
}

#[test]
fn sub() {
    let entry_data = "1.022 - 3.009".to_string();
    assert_eq!("-1.9869999999999999", &functions::entry_parser(entry_data));
    // Have to be 1.987
    let entry_data = "-1 - -2".to_string();
    assert_eq!("1", &functions::entry_parser(entry_data))
}

#[test]
fn mul() {
    let entry_data = "1.022 × 3.009".to_string();
    assert_eq!("3.0751", &functions::entry_parser(entry_data));
    // Have to be 3,075198 or 3,0752
    let entry_data = "-2 × -6".to_string();
    assert_eq!("12", &functions::entry_parser(entry_data))
}
#[test]
fn div() {
    let entry_data = "1.022 ÷ 3.009".to_string();
    assert_eq!("0.3396", &functions::entry_parser(entry_data));
    // Have to be 0,339647723
    let entry_data = "6 ÷ -2".to_string();
    assert_eq!("-3", &functions::entry_parser(entry_data))
}
