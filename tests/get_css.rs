#![allow(non_snake_case)]

extern crate userstyles;

use std::collections::HashMap;
use userstyles::get_css;

#[test]
fn with_no_settings__is_correct_css() {
    let id = 1;
    let map = HashMap::new();

    let response = get_css(id, &map);

    assert!(response.is_ok());
    assert_eq!(response.unwrap(), "*{ color: red !important; }");
}
