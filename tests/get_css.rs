#![allow(non_snake_case)]

extern crate userstyles;

use userstyles::response::{Style, StyleSetting, StyleSettingOption};
use std::collections::HashMap;

#[test]
fn with_no_settings__is_correct_css() {
    let mut style = Style::default();
    style.css = String::from("foobar");

    let response = style.get_css(None);

    assert_eq!(response, "foobar");
}

#[test]
fn with_settings__is_correct_css() {
    let mut option = StyleSettingOption::default();
    option.default = true;
    let mut settings = StyleSetting::default();
    settings.install_key = String::from("bar");
    settings.style_setting_options = vec![option];
    let mut style = Style::default();
    style.css = String::from("foo/*[[ik-bar]]*/");
    style.style_settings = vec![settings];
    let mut map = HashMap::new();
    map.insert(String::from("bar"), String::from("bar"));

    let response = style.get_css(Some(map));

    assert_eq!(response, "foobar");
}
