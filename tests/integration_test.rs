use calculator_gtk::functions;

#[test]
fn empty_input() {
    let entry_data = "".to_string();
    assert_eq!("0", functions::entry_parser(entry_data).unwrap())
}
