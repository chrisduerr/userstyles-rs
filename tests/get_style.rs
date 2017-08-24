#![allow(non_snake_case)]

extern crate userstyles;

use userstyles::response::{Style, User};
use userstyles::get_style;

#[test]
fn with_invalid_id__is_error() {
    let too_big_id = 999_999;

    let response = get_style(too_big_id);

    assert!(response.is_err());
    if let Err(e) = response {
        assert_eq!(
            e,
            "Error during API request. Expected status '200' but got '410'"
        );
    }
}

#[test]
fn with_simple_style__is_complete_struct() {
    let id = 2;
    let expected_struct = Style {
        id: 2,
        name: String::from("Go/History menu removal"),
        description: String::from(
            "Takes out the Go menu in Firefox 1.5 and the \
             History menu from Firefox 2.0. Vote for bug 313299!",
        ),
        user: User {
            id: 1,
            name: String::from("JasonBarnabe"),
            email: Some(String::from("jason.barnabe@gmail.com")),
            paypal_email: None,
            homepage: None,
            about: None,
            license: Some(String::from("publicdomain")),
        },
        updated: String::from("2006-02-10T20:57:16.000Z"),
        weekly_install_count: 0,
        total_install_count: 717,
        rating: None,
        after_screenshot_name: None,
        obsoleting_style_id: None,
        obsoleting_style_name: None,
        obsolete: 0u8,
        admin_delete_reason: None,
        obsoletion_message: None,
        screenshots: None,
        license: None,
        created: String::from("2006-02-10T20:57:16.000Z"),
        category: String::from("app"),
        pledgie_id: None,
        subcategory: Some(String::from("browser")),
        additional_info: None,
        css: String::from(
            "@namespace url(http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul);\
             \r\n#go-menu { display: none;}",
        ),
        discussions: Vec::new(),
        userjs_url: None,
        style_settings: Vec::new(),
    };

    let response = get_style(id);

    assert!(response.is_ok());
    assert_eq!(response.unwrap(), expected_struct);
}
