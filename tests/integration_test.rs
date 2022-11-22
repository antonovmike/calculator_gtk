use calculator_gtk::functions;

#[test]
fn empty() {
    let entry_data = "".to_string();
    assert_eq!("0", functions::entry_parser(entry_data))
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
    assert_eq!("4.031", &functions::entry_parser(entry_data))
}

#[test]
fn sub() {
    let entry_data = "1.022 - 3.009".to_string();
    assert_eq!("-1.9869999999999999", &functions::entry_parser(entry_data))
    // Have to be 1.987
}

#[test]
fn mul() {
    let entry_data = "1.022 × 3.009".to_string();
    assert_eq!("3.0751", &functions::entry_parser(entry_data))
    // Have to be 3,075198 or 3,0752
}
#[test]
fn div() {
    let entry_data = "1.022 ÷ 3.009".to_string();
    assert_eq!("0.3396", &functions::entry_parser(entry_data))
    // Have to be 0,339647723
}
