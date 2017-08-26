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

    let mut expected_user = User::default();
    expected_user.id = 1;
    expected_user.name = String::from("JasonBarnabe");
    expected_user.email = Some(String::from("jason.barnabe@gmail.com"));
    expected_user.license = Some(String::from("publicdomain"));
    let mut expected_style = Style::default();
    expected_style.id = 2;
    expected_style.name = String::from("Go/History menu removal");
    expected_style.description = String::from(
        "Takes out the Go menu in Firefox 1.5 and the \
         History menu from Firefox 2.0. Vote for bug 313299!",
    );
    expected_style.user = expected_user;
    expected_style.updated = String::from("2006-02-10T20:57:16.000Z");
    expected_style.total_install_count = 717;
    expected_style.created = String::from("2006-02-10T20:57:16.000Z");
    expected_style.category = String::from("app");
    expected_style.subcategory = Some(String::from("browser"));
    expected_style.css = String::from(
        "@namespace url(http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul);\
         \r\n#go-menu { display: none;}",
    );

    let response = get_style(id);

    assert!(response.is_ok());
    assert_eq!(response.unwrap(), expected_style);
}
